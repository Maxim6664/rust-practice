#include <iostream>
#include <string>

using namespace std;

string timeConversion(string s) {
string period = s.substr(8, 2); // AM or PM
int hour = stoi(s.substr(0, 2));

if (period == "AM") {
if (hour == 12)
hour = 0;
} else { // PM
if (hour != 12)
hour += 12;
}

// Format hour with two digits
char buffer[9];
sprintf(buffer, "%02d:%s", hour, s.substr(3, 5).c_str());
return string(buffer);
}

int main() {
string s;
cin >> s;
cout << timeConversion(s) << endl;
return 0;
}
