#include <cstdlib>

extern "C" {

char *dxf_to_gcode(const char *s);

void gcode_free(char *s);

}
