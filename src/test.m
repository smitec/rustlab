a = 1:10;

%% Method 1 - mex
mex rustlab.c librustlab.a
c = rustlab(a', a');

%% Method 2 - load library
if libisloaded('rl')
    unloadlibrary('rl'); % Here in case of multuple runs
end

loadlibrary('librustlab', 'rustlab.h', 'alias', 'rl');

% To pass by reference we need to create a pointer object
d = 0*a;
d = libpointer('doublePtr', d);

calllib('rl', 'multiply', a', a', d, 10);

disp(a' .* a');
disp(c);
disp(d.Value);
