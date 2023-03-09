#include <ESP8266WiFi.h>
#include <ESP8266HTTPClient.h>
#include <WiFiClient.h>
#include <ArduinoJson.h>

#include "DHT.h"
#define DHTPIN 13
#define DHTTYPE DHT11
DHT dht(DHTPIN, DHTTYPE);

//-------------------------------------------------------------------------------------------

const char* ssid = "Privat W-LAN";
const char* password = "99842DA5D0";

const char* serverAddress = "http://api.temperature-station.lodner.dev/measurement";

void setup() {
  Serial.begin(9600);
  //Serial.begin(115200);

  WiFi.begin(ssid, password);
  Serial.println("Connecting");

  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }

  Serial.println("");
  Serial.print("Connected to WiFi network with IP Address: ");
  Serial.println(WiFi.localIP());

  dht.begin();
}

void loop() {
  // put your main code here, to run repeatedly:
  if (WiFi.status() == WL_CONNECTED) {
    WiFiClient client;
    HTTPClient http;

    float humidity = dht.readHumidity();
    float temperature = dht.readTemperature();

    if (isnan(humidity) || isnan(temperature)) {
      Serial.println("Fehler beim auslesen des Sensors!");
      return;
    }

    Serial.print("Luftfeuchtigkeit: ");
    Serial.print(humidity);  // Ausgeben der Luftfeuchtigkeit
    Serial.print("%\t");     // Tabulator
    Serial.print("Temperatur: ");
    Serial.print(temperature);  // Ausgeben der Temperatur
    Serial.write('°');          // Schreiben des ° Zeichen
    Serial.println("C");

    // Create json
    Serial.println("Create JSON");
    //const int capacity = JSON_OBJECT_SIZE(192);
    StaticJsonDocument<200> doc;
    doc["id"] = 0;
    doc["room"] = "test";
    doc["device"] = "test";
    doc["date_time"] = "2021-11-03T15:13:39.259609+00:00";
    doc["temperature"].set(temperature);
    doc["humidity"].set(humidity);

    String output = "";
    serializeJson(doc, output);

    Serial.println(output);

    Serial.println("Json end.");

    // Your Domain name with URL path or IP address with path
    http.begin(client, serverAddress);

    // If you need an HTTP request with a content type: application/json, use the following:
    http.addHeader("Content-Type", "application/json");
    int httpCode = http.POST(output);

    Serial.print("HTTP Response code: ");
    Serial.println(httpCode);

    // httpCode will be negative on error
    if (httpCode > 0) {
      // HTTP header has been send and Server response header has been handled
      Serial.printf("[HTTP] POST... code: %d\n", httpCode);

      // file found at server
      if (httpCode == HTTP_CODE_OK) {
        const String& payload = http.getString();
        Serial.println("received payload:\n<<");
        Serial.println(payload);
        Serial.println(">>");
      }
    } else {
      Serial.printf("[HTTP] POST... failed, error: %s\n", http.errorToString(httpCode).c_str());
    }

    // Free resources
    http.end();

    delay(30000);
  } else {
    Serial.println("WiFi Disconnected");
  }
}
