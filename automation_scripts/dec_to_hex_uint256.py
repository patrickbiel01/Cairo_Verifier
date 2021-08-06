# Used to make test data code
# From a list of new-line seperated decimal number
	# e.g Copied and pasted from etherscan
	#10
	#5
	#...
# Converts to a Uint256 compatible with the library

f = open("cairo_aux_input.txt", "r")
text = f.read()
newText = ""

parsedNum = ""
i = 0
newLineStartIdx = 0

# print(len(text))

while i < len(text):

	# print(text[i]);

	if text[i] == '\n':
		# print(parsedNum);
		# Do whatever with this
		decimal_num = int(parsedNum, 10)
		newText += "cairo_verifier::uint256_ops::get_uint256(\"" + hex(decimal_num)[2:] + "\"),\n"

		# Reset
		parsedNum = ""
		newLineStartIdx = i
		i += 1
		continue

	parsedNum += str(text[i]);
	i += 1

f.close()

print(newText)
