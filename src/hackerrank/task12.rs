#include <iostream>
#include <vector>
using namespace std;

int main() {
int s, t; // house range
int a, b; // apple tree and orange tree positions
int m, n; // number of apples and oranges

cin >> s >> t;
cin >> a >> b;
cin >> m >> n;

vector<int> apples(m);
vector<int> oranges(n);

for (int i = 0; i < m; ++i) {
cin >> apples[i];
}

for (int i = 0; i < n; ++i) {
cin >> oranges[i];
}

int apple_count = 0;
int orange_count = 0;

for (int i = 0; i < m; ++i) {
int landing_position = a + apples[i];
if (landing_position >= s && landing_position <= t) {
++apple_count;
}
}

for (int i = 0; i < n; ++i) {
int landing_position = b + oranges[i];
if (landing_position >= s && landing_position <= t) {
++orange_count;
}
}

cout << apple_count << endl;
cout << orange_count << endl;

return 0;
}
