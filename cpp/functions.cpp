#include <iostream>
#include <vector>

using namespace std;

vector<double> linspace(double start, double end, int numPoints) {
    vector<double> result(numPoints);
    double step = (end - start) / (numPoints - 1);
    for (int i = 0; i < numPoints; ++i) {
        result[i] = start + i * step;
    }
    return result;
}

int main(){
  vector<double> x = linspace(-5,5,1000);
  int y = 1;
  for (double value : x){
    cout << y << endl;
    y++;
  }
}
