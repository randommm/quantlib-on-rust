/*!

 Based on
 https://github.com/lballabio/QuantLib/blob/master/Examples/MultidimIntegral/MultidimIntegral.cpp
 which has the following license:
 https://github.com/lballabio/QuantLib/blob/master/LICENSE.TXT

*/

#pragma once
#include "quantlib-on-rust/include/some_example.h"
#include "quantlib-on-rust/src/main.rs.h"

#include <ql/qldefines.hpp>
#if !defined(BOOST_ALL_NO_LIB) && defined(BOOST_MSVC)
#include <ql/auto_link.hpp>
#endif
#include <ql/experimental/math/multidimintegrator.hpp>
#include <ql/experimental/math/multidimquadrature.hpp>
#include <ql/functional.hpp>
#include <ql/math/integrals/trapezoidintegral.hpp>
#include <ql/patterns/singleton.hpp>

#include <iomanip>
#include <iostream>

using namespace QuantLib;
using namespace std;

// Correct value is: (e^{-.25} \sqrt{\pi})^{dimension}
struct integrand {
  Real operator()(const std::vector<Real> &arg) const {
    Real sum = 1.;
    for (Real i : arg)
      sum *= std::exp(-i * i) * std::cos(i);
    return sum;
  }
};

int run_integral() {

  try {

    std::cout << std::endl;

    Size dimension = 3;
    Real exactSol = std::pow(std::exp(-.25) * std::sqrt(M_PI),
                             static_cast<Real>(dimension));

    ext::function<Real(const std::vector<Real> &arg)> f = integrand();

#ifndef QL_PATCH_SOLARIS
    GaussianQuadMultidimIntegrator intg(dimension, 15);

    Real valueQuad = intg(f);
#endif

    std::vector<ext::shared_ptr<Integrator>> integrals;
    for (Size i = 0; i < dimension; i++)
      integrals.push_back(
          ext::make_shared<TrapezoidIntegral<Default>>(1.e-4, 20));
    std::vector<Real> a_limits(integrals.size(), -4.);
    std::vector<Real> b_limits(integrals.size(), 4.);
    MultidimIntegral testIntg(integrals);

    Real valueGrid = testIntg(f, a_limits, b_limits);

    cout << fixed << setprecision(4);
    cout << endl
         << "-------------- " << endl
         << "Exact: " << exactSol << endl
#ifndef QL_PATCH_SOLARIS
         << "Quad: " << valueQuad << endl
#endif
         << "Grid: " << valueGrid << endl
         << endl;

    return 0;

  } catch (std::exception &e) {
    std::cerr << e.what() << std::endl;
    return 1;
  } catch (...) {
    std::cerr << "unknown error" << std::endl;
    return 1;
  }
}
