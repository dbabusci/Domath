The ‘domath’ (pronounced Dometh) is a terminal command, made with rust, used for evaluating simple mathematical expressions.

Currently parenthesis are broken for the build due to bash errors. Therefore the operators ‘(’ and ‘)’ are replaced by ‘p’ and ‘d’.

Example operations:
	domath 1+1 -> 2
	domath 1-1 -> 0
	domath 1*1 -> 1
	domath 1/1 -> 1
	domath 1^1 -> 1
domath p1+1d -> 2
	domath (1+1) -> error with bash
	cargo run (1+1) -> 2 (while not built)
	
Future features:
	Add a way to display tests
	Add a way to take in text file and output results in text file
	Fix bash error with ()