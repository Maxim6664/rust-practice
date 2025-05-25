#include <iostream>
#include <algorithm>

using namespace std;

int pageCount(int n, int p) {
int fromFront = p / 2;
int fromBack = (n / 2) - (p / 2);
return min(fromFront, fromBack);
}

int main() {
int n, p;
cin >> n >> p;
cout << pageCount(n, p) << endl;
return 0;
}