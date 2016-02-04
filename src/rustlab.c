#include "mex.h"

// Multipleies a and b element wise, and puts the result in c
extern void multiply(double* a, double* b, double* c, long elements);

void mexFunction(int nlhs, mxArray *plhs[], 
        int nrhs, const mxArray *prhs[]) {
    double* a;
    double* b;
    double* c;

    mwSize elements;

    if (nrhs != 2) {
        mexErrMsgTxt("Wrong number of input args");
    }

    if (nlhs != 1) {
        mexErrMsgTxt("Wrong number of output args");
    }

    a = mxGetPr(prhs[0]);
    b = mxGetPr(prhs[1]);
    elements = mxGetM(prhs[0]);

    plhs[0] = mxCreateDoubleMatrix(elements, 1, mxREAL);
    c = mxGetPr(plhs[0]);

    multiply(a, b, c, elements);
}
