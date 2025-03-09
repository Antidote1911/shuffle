#ifndef ADAPTER_H
#define ADAPTER_H

#include <QFileDialog>
#include <QString>

// rust functions
extern "C" char* get_version();
extern "C" char* get_random(int length,
                            bool uppercase,
                            bool lowwercase,
                            bool digits,
                            bool symbols);



#endif // ADAPTER_H
