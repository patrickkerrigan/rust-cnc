#pragma once

#ifdef __cplusplus
extern "C" {
#endif

char *dxf_to_gcode(const char *s);

void gcode_free(char *s);

#ifdef __cplusplus
}
#endif
