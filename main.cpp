using namespace std;

#include <iostream>
#include <fstream>
#include <cmath>
#include <tuple>
#include <functional>

const float PI = 3.14159265352;

void write_solution(float c_1_exact, float c_2_exact, float c_1_halve, float c_2_halve, float c_1_newton, float c_2_newton)
{
	string head{R"(set terminal qt size 1000,1000 enhanced font 'Verdana,10' persist
# Line width of the axes
set border linewidth 1.5
# Line styles
set style line 1 linecolor rgb '#0060ad' linetype 1 linewidth 2
set style line 2 linecolor rgb '#dd181f' linetype 1 linewidth 2
set style line 3 linecolor rgb '#00d100' linetype 1 linewidth 2
# Axes label
set xlabel 'x'
set ylabel 'y'
# Axes ranges
set xrange [0:0.5*pi]
set xtics ('0' 0, 'π/8' 0.125 * pi, 'π/4' 0.25 * pi, '3π/8' 0.375 * pi, 'π/2' 0.5 * pi)
# Paramters
c_1_exact = )"};

    string line1{R"(
c_2_exact = )"};

    string line2{R"(
c_1_halve = )"};

    string line3{R"(
c_2_halve = )"};

	string line4{R"(
c_1_newton = )"};

	string line5{R"(
c_2_newton = )"};

    string tail{R"(
# Fuction
exact(x) = c_1_exact * exp(4 * x) + c_2_exact * exp(-4 * x) + 0.5
halve(x) = c_1_halve * exp(4 * x) + c_2_halve * exp(-4 * x) + 0.5
newton(x) = c_1_newton * exp(4 * x) + c_2_newton * exp(-4 * x) + 0.5
# Plot
plot exact(x) title 'exaktní řešení' with lines linestyle 1, \
     halve(x) title 'Metoda půlení intervalů' with lines linestyle 2, \
	 newton(x) title 'Newtonova metoda' with lines linestyle 3)"};

	ofstream file{"./solution.gnu"};

	file << head << c_1_exact << line1 << c_2_exact << line2 << c_1_halve << line3 << c_2_halve << line4 << c_1_newton << line5 << c_2_newton << tail;
}

tuple<float, float> exact(float alpha, float beta)
{
	float c_1{((2.0f * beta - 1.0f) + exp(-2.0f * PI) * (1.0f - 2.0f * alpha)) / (4.0f * sinh(2.0f * PI))};
	float c_2{alpha - 0.5f - c_1};
	return make_tuple(c_1, c_2);
}


tuple<float, float> halve(float alpha, float beta, int rounds)
{
	function<float(float, float)> c_1{[](float a, float c)->float{return 0.5 * a + 0.125 * c - 0.25;}};
	function<float(float, float)> c_2{[](float a, float c)->float{return 0.5 * a - 0.125 * c - 0.25;}};

	function<float(float)> F{[alpha, beta, c_1, c_2](float c)->float{return c_1(alpha, c) * exp(2.0 * PI) + c_2(alpha, c) * exp(-2.0 * PI) + 0.5 - beta;}};

	float gamma, gamma_left, gamma_right;

	for(int i = 0; i < 100000; ++i)
	{
		gamma_left = static_cast<float>(rand()) / static_cast<float>(RAND_MAX/2000) - 1000;
		gamma_right = static_cast<float>(rand()) / static_cast<float>(RAND_MAX/2000) - 1000;
		if(F(gamma_left) * F(gamma_right) < 0.0)
		{
			break;
		}
	}
	if(F(gamma_left) * F(gamma_right) >= 0.0)
	{
		cout << "Nepodařilo se najít počáteční stav metody půlení intervalů" << endl;
		abort();
	}

	cout << "Půlím..." << endl;

	for(int i = 0; i < rounds; ++i)
	{
		gamma = 0.5 * (gamma_left + gamma_right);
		float middle = F(gamma);
		if(middle * F(gamma_right) > 0.0)
		{
			gamma_right = gamma;
		}
		else
		{
			gamma_left = gamma;
		}
	}
	return make_tuple(c_1(alpha, gamma), c_2(alpha, gamma));
}

tuple<float, float> newton(float alpha, float beta, int rounds)
{
	function<float(float, float)> c_1{[](float a, float c)->float{return 0.5 * a + 0.125 * c - 0.25;}};
	function<float(float, float)> c_2{[](float a, float c)->float{return 0.5 * a - 0.125 * c - 0.25;}};

	function<float(float)> f{[alpha, beta, c_1, c_2](float c)->float{return c_1(alpha, c) * exp(2.0 * PI) + c_2(alpha, c) * exp(-2.0 * PI) + 0.5 - beta;}};
	function<float(float)> f_der{[](float)->float{return 0.125 * exp(2.0 * PI) - 0.125 * exp(-2.0 * PI);}};

	float gamma{static_cast<float>(rand()) / static_cast<float>(RAND_MAX/2000) - 1000};

	for(int i = 0; i < rounds; ++i)
	{
		gamma = gamma - f(gamma) / f_der(gamma);
	}
	return make_tuple(c_1(alpha, gamma), c_2(alpha, gamma));
}

int main()
{
	float alpha;
	float beta;
	int rounds;
	cout << "Zadejte parametr alfa: " << endl;
	cin >> alpha;
	cout << "Zadejte parametr beta: " << endl;
	cin >> beta;
	cout << "Zadejte počet iterací: " << endl;
	cin >> rounds;

	tuple<float, float> c_exact = exact(alpha, beta);
	tuple<float, float> c_halve = halve(alpha, beta, rounds);
	tuple<float, float> c_newton = newton(alpha, beta, rounds);

	write_solution(get<0>(c_exact), get<1>(c_exact), get<0>(c_halve), get<1>(c_halve), get<0>(c_newton), get<1>(c_newton));
}
