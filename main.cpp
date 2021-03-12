#include <cstdlib>
#include <iostream>
#include <ctime>

#define VERSION "V0.1"
#define MIN_NUM 1
#define MAX_NUM 100

using namespace std;

bool checkGuess(int guess, int random, int guesses) {
    if (guess == random) {
        cout << "  Correct!" << endl;
        cout << "  You won in " << guess << " guesses!", guesses + 1;
        return false;
    } else if (guess > random) {
        cout << "  Go Lower" << endl;
    } else if (guess < random) {
        cout << "  Go Higher" << endl;
    }

    return true;
}

int main() {
    srand(time(nullptr));
    int random = MIN_NUM + (rand() % static_cast<int>(MAX_NUM - MIN_NUM + 1));

    cout << "debug: " << random << endl;

    cout << "Welcome to my Guessing Game " << VERSION << endl;
    cout << "Enter a Guess Between " << MIN_NUM << " and " << MAX_NUM << endl;
    cout << "Good Luck" << endl << endl;
    bool running = true;
    string input;
    int guesses = 0;
    while (running) {
        cout << ">>>";
        cin >> input;
        running = checkGuess(stoi(input), random, guesses);
        guesses++;
    }

    return 0;
}