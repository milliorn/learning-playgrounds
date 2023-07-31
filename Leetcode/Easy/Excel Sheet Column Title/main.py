class Solution:
    def convertToTitle(self, columnNumber: int) -> str:
        result = ""
        while columnNumber > 0:
            # Get the remainder after dividing columnNumber by 26
            remainder = (columnNumber - 1) % 26

            # Convert the remainder to a character and add it to the result
            result = chr(remainder + ord('A')) + result

            # Update columnNumber by subtracting the remainder and dividing by 26
            columnNumber = (columnNumber - 1) // 26

        return result


def print_hi(name):
    # Use a breakpoint in the code line below to debug your script.
    print(f'Hi, {name}')  # Press Ctrl+F8 to toggle the breakpoint.


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    print_hi('PyCharm')

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
