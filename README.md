 Generating the bindings

 ```
 echo '#include <magick/api.h>' &&
 ./bindgen `GraphicsMagick-config --cppflags --ldflags --libs` ~/gen.h -o ../graphicksmagic-rs/src/bindings.rs -builtins
 ```
