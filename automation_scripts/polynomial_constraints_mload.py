# Script to help automation of converting a cpu/layoutX/CpuConstraintPoly.sol contract to Rust
# Parses out "mload(0xDEADBEEF)" (Solidity EVM Assembley Command) from the file and replaces it with the proper Rust equivalent

# Note: replace file_name.txt appropriately
# file_name.txt should contain Solidity Assembly from CpuConstraintPoly.sol
# Further Reductions example: You can reduce addmod(x, y, PRIME) to prime_field::fadd(x.clone(), y.clone());
# by using Find and Replace in an IDE. eg. Find: ", PRIME)" Replace with: ")"

f = open("file_name.txt", "r")
text = f.read()

parsedNum = ""
startParsing = False

i = 5
mloadStartIdx = -1

# Searches for "mload(" and reads the hex address until it reaches the closing bracket, ")"
# Once the hex address is parsed the mload(0xDEADBEEF) is replaced with a proper eqivalent
# derived from the memory mapping at the top of CpuConstraintPoly.sol
# The new read is placed in mload(0xDEADBEEF)'s spot in the file

while i < len(text):

	if text[i] == ')' and startParsing:
		addr = int(parsedNum, 0)
		newText = ""

		if addr == 0 :
			newText += "ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__X].clone()"
		elif addr == 32 :
			newText += "ctx[map::MM_PERIODIC_COLUMN__PEDERSEN__POINTS__Y].clone()"
		elif addr == 64 :
			newText += "ctx[map::MM_PERIODIC_COLUMN__ECDSA__GENERATOR_POINTS__X].clone()"
		elif addr == 96 :
			newText += "ctx[map::MM_PERIODIC_COLUMN__ECDSA__GENERATOR_POINTS__Y].clone()"
		elif addr == 128 :
			newText += "ctx[map::MM_TRACE_LENGTH].clone()"
		elif addr == 160 :
			newText += "ctx[map::MM_OFFSET_SIZE].clone()"
		elif addr == 192 :
			newText += "ctx[map::MM_HALF_OFFSET_SIZE].clone()"
		elif addr == 224 :
			newText += "ctx[map::MM_INITIAL_AP].clone()"
		elif addr == 256 :
			newText += "ctx[map::MM_INITIAL_PC].clone()"
		elif addr == 288 :
			newText += "ctx[map::MM_FINAL_AP].clone()"
		elif addr == 320 :
			newText += "ctx[map::MM_FINAL_PC].clone()"
		elif addr == 352 :
			newText += "ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__PERM__INTERACTION_ELM].clone()"
		elif addr == 384 :
			newText += "ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__HASH_INTERACTION_ELM0].clone()"
		elif addr == 416 :
			newText += "ctx[map::MM_MEMORY__MULTI_COLUMN_PERM__PERM__PUBLIC_MEMORY_PROD].clone()"
		elif addr == 448 :
			newText += "ctx[map::MM_RC16__PERM__INTERACTION_ELM].clone()"
		elif addr == 480 :
			newText += "ctx[map::MM_RC16__PERM__PUBLIC_MEMORY_PROD].clone()"
		elif addr == 512 :
			newText += "ctx[map::MM_RC_MIN].clone()"
		elif addr == 544 :
			newText += "ctx[map::MM_RC_MAX].clone()"
		elif addr == 576 :
			newText += "ctx[map::MM_PEDERSEN__SHIFT_POINT_X].clone()"
		elif addr == 608 :
			newText += "ctx[map::MM_PEDERSEN__SHIFT_POINT_Y].clone()"
		elif addr == 640 :
			newText += "ctx[map::MM_INITIAL_PEDERSEN_ADDR].clone()"
		elif addr == 672 :
			newText += "ctx[map::MM_INITIAL_RC_ADDR].clone()"
		elif addr == 704 :
			newText += "ctx[map::MM_ECDSA__SIG_CONFIG_ALPHA].clone()"
		elif addr == 736 :
			newText += "ctx[map::MM_ECDSA__SIG_CONFIG_SHIFT_POINT_X].clone()"
		elif addr == 768 :
			newText += "ctx[map::MM_ECDSA__SIG_CONFIG_SHIFT_POINT_Y].clone()"
		elif addr == 800 :
			newText += "ctx[map::MM_ECDSA__SIG_CONFIG_BETA].clone()"
		elif addr == 832 :
			newText += "ctx[map::MM_INITIAL_ECDSA_ADDR].clone()"
		elif addr == 864 :
			newText += "ctx[map::MM_TRACE_GENERATOR].clone()"
		elif addr == 896 :
			newText += "ctx[map::MM_OODS_POINT].clone()"
		elif addr == 928 :
			newText += "ctx[map::MM_INTERACTION_ELEMENTS+0].clone()"
		elif addr == 960 :
			newText += "ctx[map::MM_INTERACTION_ELEMENTS+1].clone()"
		elif addr == 992 :
			newText += "ctx[map::MM_INTERACTION_ELEMENTS+2].clone()"
		elif addr < 12480 and addr >= 1024 :
			newText += "ctx[map::MM_COEFFICIENTS+" + str( int((addr-1024)/32) ) + "].clone()"
		elif addr < 18880 and addr >= 12480 :
			newText += "ctx[map::MM_OODS_VALUES+" + str( int((addr-12480)/32) ) + "].clone()"
		elif addr < 20256 and addr >= 18880 :
			newText += "intermediate_vals[" + str( int((addr-18880)/32) ) + "].clone()"
		elif addr < 20928 and addr >= 20256 :
			newText += "exp_mods[" + str( int((addr-20256)/32) ) + "].clone()"
		elif addr < 21632 and addr >= 20928 :
			newText += "denominator_inv[" + str( int((addr-20928)/32) ) + "].clone()"
		elif addr < 22336 and addr >= 21632 :
			newText += "denominators[" + str( int((addr-21632)/32) ) + "].clone()"
		elif addr < 22656 and addr >= 22336 :
			newText += "numerators[" + str( int((addr-22336)/32) ) + "].clone()"
		else :
			# Reset
			parsedNum = ""
			startParsing = False
			mloadStartIdx = -1
			i += 1
			continue

		text = text[:mloadStartIdx] + newText + text[i+1:]

		# Reset
		parsedNum = ""
		startParsing = False
		mloadStartIdx = -1
		i += 1
		continue
	if startParsing :
		parsedNum += str(text[i])
		i += 1
		continue
	if text[i] == '(' and text[i-1] == 'd' and text[i-2] == 'a' and text[i-3] == 'o' and text[i-4] == 'l' and text[i-5] == 'm':
		startParsing = True
		mloadStartIdx =i-5
	i += 1

f.close()

f = open("file_name.txt", "w")
f.write(text)
f.close()

print(text)
