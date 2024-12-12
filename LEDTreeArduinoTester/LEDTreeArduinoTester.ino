#include <FastLED.h>

#define LED_COUNT 300

#define DATA_PIN 2

CRGB leds[LED_COUNT];

void setup() {
  Serial.begin(115200);
  while (!Serial) { ; }
  // Serial.write("Connected\n");
  // Serial.setTimeout(1000);

  FastLED.addLeds<WS2812B, DATA_PIN, RGB>(leds, LED_COUNT);

  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, HIGH);
}

int currentLED = -1;

void clearSerial() {
  while (Serial.available() > 0) Serial.read();
}

void sendMsg(char msg) {
  delay(20);
  Serial.print(msg);
}

void loop() {
  if (Serial.available() > 0) {
    char code = Serial.read();

    switch (code) {
      case 'N':
        if (currentLED < LED_COUNT - 1) {
          // Serial.println("Next");
          // Turn on LED, Turn off previous
          if (currentLED >= 0)
            leds[currentLED] = CRGB::Black;
          FastLED.show();
          leds[currentLED + 1] = CRGB::Blue;
          FastLED.show();
          sendMsg('S');
        } else {
          // Serial.println("Must Reset");

          leds[currentLED] = CRGB::Black;
          sendMsg('F');
        }
        currentLED += 1;
        break;
      case 'R':
        for (int i = 0; i < LED_COUNT; i++)
          leds[i] = CRGB::Black;
        currentLED = -1;
        // Serial.println("Reset");

        sendMsg('R');
        break;
    }
    clearSerial();
    // Serial.print("| CODE: ");
    // Serial.println(code);
  }
}
