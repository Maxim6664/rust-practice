#include <iostream>
#include <vector>
using namespace std;

void bonAppetit(const vector<int>& bill, int k, int b) {
int total = 0;
for (int i = 0; i < bill.size(); ++i) {
if (i != k) {
total += bill[i];
}
}

int anna_share = total / 2;

if (b == anna_share) {
cout << "Bon Appetit" << endl;
} else {
cout << b - anna_share << endl;
}
}

int main() {
int n, k;
cin >> n >> k;
vector<int> bill(n);

for (int i = 0; i < n; ++i) {
cin >> bill[i];
}

int b;
cin >> b;

bonAppetit(bill, k, b);
return 0;
}
