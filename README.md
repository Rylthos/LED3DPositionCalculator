# 3D LED Position Calculator and Controller

## Effects
Currently implemented are
- Solid Colour
- Moving Rainbow up the y Axis
- Random planes moving in any direction, with randomized colour
- An expanding sphere from the centre

Each effect has settings that can be modified.
All settings that can be changed are saved and are reloaded when the program is opened again

## How to use
### 3D Position
1. Flash an arduino with the provided LED3DPositionCalculator code
  - Make sure the LED count and data pin are set correctly
2. Connect this arduino to a computer with a webcam
3. Position the leds in such a way that they are all visible in the camera's view and roughly centered in the view
4. Ensure the room is dark and then on the connected computer run
```bash
python3 LEDPicture.py
```
5. When prompted to rotate the object, trying to make sure the center remains roughly in the same place
6. Repeat this until you have rotated the object 360 degrees.
7. After all the pictures are taken run
```bash
python3 3DPositionCalculator.py
```
8. After this has finished executing you should be able to see a plot with the generated information, you can view this again by executing
```bash
python3 DataViewer.py
```
9. The generated Output.pixels can be copied into the LEDController file

### Controller
1. The Arduino now needs to be flashed with something that can handle ddp, I personally recommend [WLED](https://kno.wled.ge/)
2. Ensure the IP and number of leds set in src/main.rs is set correctly
3. Ensure the constants set in src/effects/constants.rs are valid, in particular the min and max coordinates
4. Run
```bash
cargo run
```
to start the controller

