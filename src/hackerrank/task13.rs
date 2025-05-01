#include <iostream>
using namespace std;

string kangaroo(int x1, int v1, int x2, int v2) {
// If velocities are equal
if (v1 == v2) {
return (x1 == x2) ? "YES" : "NO";
}

int numerator = x2 - x1;
int denominator = v1 - v2;

// Check if they will land together after the same number of jumps
if (denominator != 0 && (numerator % denominator == 0) && (numerator / denominator >= 0)) {
return "YES";
}

return "NO";
}

int main() {
int x1, v1, x2, v2;
cin >> x1 >> v1 >> x2 >> v2;

cout << kangaroo(x1, v1, x2, v2) << endl;

return 0;
}
