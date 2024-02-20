#pragma once
#include "rust/cxx.h"
#include <memory>

int run_integral(rust::Fn<double(rust::Vec<double> arg)> to_integrate);
