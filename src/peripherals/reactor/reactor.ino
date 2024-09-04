#include <ESP8266WiFi.h>
#include "Adafruit_MQTT.h"
#include "Adafruit_MQTT_Client.h"

#define WLAN_SSID       ""
#define WLAN_PASS       ""
#define AIO_SERVER      "192.168.0.92"
#define AIO_SERVERPORT  1883
#define PERIPHERAL_NAME "Reactor"
#define REPAIR_BTN 5

#define DEBUG false

WiFiClient client;
Adafruit_MQTT_Client mqtt(&client, AIO_SERVER, AIO_SERVERPORT);
Adafruit_MQTT_Publish r_publish = Adafruit_MQTT_Publish(&mqtt, PERIPHERAL_NAME);
Adafruit_MQTT_Subscribe r_sub = Adafruit_MQTT_Subscribe(&mqtt, PERIPHERAL_NAME);

unsigned long keyPrevMillis = 0;
const unsigned long keySampleIntervalMs = 25;
byte longKeyPressCountMax = 80;    // 80 * 25 = 2000 ms
byte longKeyPressCount = 0;
byte prevKeyState = LOW;

void MQTT_connect();

void setup() {
  if (DEBUG) {
    Serial.begin(115200);
    delay(10);

    Serial.println(F("Adafruit MQTT demo"));

    // Connect to WiFi access point.
    Serial.println(); Serial.println();
    Serial.print("Connecting to ");
    Serial.println(WLAN_SSID);
  }

  pinMode(REPAIR_BTN, INPUT);

  WiFi.begin(WLAN_SSID, WLAN_PASS);
  while (WiFi.status() != WL_CONNECTED) {
    delay(100);
  }
  if (DEBUG) {
    Serial.println("WiFi connected");
    Serial.println("IP address: "); Serial.println(WiFi.localIP());
  }
  mqtt.subscribe(&r_sub);
}

void loop() {
  MQTT_connect();

  // this is our 'wait for incoming subscription packets' busy subloop
  // try to spend your time here
  //Adafruit_MQTT_Subscribe *subscription;
  //while ((subscription = mqtt.readSubscription(5000))) {
  //  if (subscription == &onoffbutton) {
  //    Serial.print(F("Got: "));
  //    Serial.println((char *)onoffbutton.lastread);
  //  }
  //}

  if (millis() - keyPrevMillis >= keySampleIntervalMs) {
    keyPrevMillis = millis();

    byte currKeyState = digitalRead(REPAIR_BTN);

    if ((prevKeyState == HIGH) && (currKeyState == LOW)) {
      keyPress();
    }
    else if ((prevKeyState == LOW) && (currKeyState == HIGH)) {
      keyRelease();
    }
    else if (currKeyState == LOW) {
      longKeyPressCount++;
    }

    prevKeyState = currKeyState;
  }



  if (! mqtt.ping()) {  //ping the server to keep the mqtt connection alive
    mqtt.disconnect();
  }
}

void longKeyPress() {
  if (DEBUG) { Serial.println("send"); }
  r_publish.publish("{\"Reactor\":\"Repair\"}");
}

void keyRelease() {
  if(DEBUG){Serial.println("keyrelease");}

  if (longKeyPressCount >= longKeyPressCountMax) {
    longKeyPress();
  }
}

void keyPress() {
  if(DEBUG){Serial.println("keypress");}
  longKeyPressCount = 0;
}

// Function to connect and reconnect as necessary to the MQTT server.
// Should be called in the loop function and it will take care if connecting.
void MQTT_connect() {
  int8_t ret;

  // Stop if already connected.
  if (mqtt.connected()) {
    return;
  }

  if (DEBUG) {
    Serial.print("Connecting to MQTT... ");
  }

  uint8_t retries = 3;
  while ((ret = mqtt.connect()) != 0) { // connect will return 0 for connected
    if (DEBUG) {
      Serial.println(mqtt.connectErrorString(ret));
      Serial.println("Retrying MQTT connection in 5 seconds...");
    }
    mqtt.disconnect();
    delay(5000);  // wait 5 seconds
    retries--;
    if (retries == 0) {
      // basically die and wait for WDT to reset me
      while (1);
    }
  }
  if (DEBUG) {
    Serial.println("MQTT Connected!");
  }
}
