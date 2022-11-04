// SnykEventService provides operations for snyk events.
package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"time"

	aad "github.com/Azure/azure-amqp-common-go/v3/aad"
	eventhubs "github.com/Azure/azure-event-hubs-go/v3"
	"github.com/go-kit/kit/endpoint"
	httptransport "github.com/go-kit/kit/transport/http"
)

type SnykEvent struct {
	TimeStamp (time.Time)
	Value     (map[string]interface{})
	Headers   (map[string][]string)
}

type SnykEventService interface {
	Store(SnykEvent) (string, error)
}

type snykEventService struct{}

func (snykEventService) Store(event SnykEvent) (string, error) {
	return event.TimeStamp.String(), nil
}

type storeRequest struct {
	Event   (map[string]interface{})
	Headers (map[string][]string)
}

type storeResponse struct {
	Event (map[string]interface{})
}

func postToElastic(body map[string]interface{}) {
	elasticUrl := os.Getenv("ELASTIC_URL")
	elasticPassword := os.Getenv(("ELASTIC_PASSWORD"))
	elasticUser := os.Getenv("ELASTIC_USER")
	data, error := json.Marshal(body)
	if error != nil {
		log.Fatal("Could not marshal data", error)
	}
	req, error := http.NewRequest(
		"POST", elasticUrl, bytes.NewBuffer(data),
	)
	if error != nil {
		log.Printf("Creating request failed %s\n", error)
	}

	req.SetBasicAuth(elasticUser, elasticPassword)
	req.Header.Add("Content-Type", "application/json")
	client := &http.Client{}
	resp, error := client.Do(req)
	log.Print(resp)
}

func makeStoreEndpoint(ses SnykEventService) endpoint.Endpoint {
	tokenProvider, err := aad.NewJWTProvider(aad.JWTProviderWithEnvironmentVars())
	if err != nil {
		log.Fatalf("failed to configure AAD JWT provider: %s\n", err)
	}
	hub, err := eventhubs.NewHub("snyk-events", "snyk-events", tokenProvider)
	if err != nil {
		log.Fatalf("failed to get hub %s\n", err)
	}

	return func(_ context.Context, request interface{}) (interface{}, error) {
		println("Processing request")
		req := request.(storeRequest)
		snykEvent := SnykEvent{time.Now(), req.Event, req.Headers}
		eventValue, err := json.Marshal(snykEvent)
		// ctx, cancel := context.WithTimeout(context.Background(), 100*time.Millisecond)
		ctx, cancel := context.WithCancel(context.Background())
		event := eventhubs.NewEventFromString(string(eventValue))
		hub.Send(ctx, event)
		defer cancel()
		defer hub.Close(ctx)
		println("Sent event")
		err = hub.Close(context.Background())
		if err != nil {
			fmt.Println(err)
		}

		postToElastic(req.Event)
		return storeResponse{req.Event}, nil
	}
}

func makePingEndpoint(ses snykEventService) endpoint.Endpoint {
	return func(_ context.Context, request interface{}) (interface{}, error) {
		retval := map[string]string{}
		retval["ok"] = "true"
		return retval, nil
	}
}

func main() {
	ses := snykEventService{}

	storeSnykEventHandler := httptransport.NewServer(
		makeStoreEndpoint(ses),
		decodeStoreRequest,
		encodeStoreResponse,
	)

	pingEventHandler := httptransport.NewServer(
		makePingEndpoint(ses),
		decodePingRequest,
		encodeStoreResponse,
	)

	http.Handle("/store", storeSnykEventHandler)
	http.Handle("/ping", pingEventHandler)
	log.Fatal(http.ListenAndServe(":8080", nil))
}

func decodeStoreRequest(_ context.Context, r *http.Request) (interface{}, error) {
	var request storeRequest
	if err := json.NewDecoder(r.Body).Decode(&request.Event); err != nil {
		println("Could not decode payload")
		println(request.Event)
		return nil, err
	}
	request.Headers = r.Header
	return request, nil
}

func decodePingRequest(_ context.Context, r *http.Request) (interface{}, error) {
	return "", nil
}

func encodeStoreResponse(_ context.Context, w http.ResponseWriter, response interface{}) error {
	return json.NewEncoder(w).Encode(response)
}
