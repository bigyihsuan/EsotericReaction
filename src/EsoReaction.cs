using System;
using System.IO;
using System.Collections;
using System.Collections.Generic;
using Interpreter.Lexer;

namespace Interpreter {
    public class EsoReaction {
        static bool hadError = false;
        public static bool debug = false;
        public static bool printStack = false;

        public static void Main(string[] args) {
            if (args.Length == 0) {
                Console.WriteLine("Insufficient arguments: " + args.Length);
                Console.WriteLine("Usage: EsoReaction [OPTIONS] <filePath>");
                Console.WriteLine("    OPTIONS = ");
                Console.WriteLine("        -d : Debug");
                Console.WriteLine("        -s : Print Stack");
                Environment.Exit(64);
            } else {
                debug = Array.Exists<string>(args, str => str == "-d");
                printStack = Array.Exists<string>(args, str => str == "-s");
                int fileIndex = 0;
                for (; fileIndex < args.Length; fileIndex++) {
                    if (args[fileIndex] != "-d" && args[fileIndex] != "-s") {
                        break;
                    }
                }
                Run(args[fileIndex]);
                if (hadError) Environment.Exit(65);
            }
        }

        static void Run(string file) {
            Lexer.Lexer lexer = new Lexer.Lexer(File.ReadAllText(file));
            List<Token> tokens = lexer.ScanTokens();

            foreach (var t in tokens) {
                Console.WriteLine(t);
            }
        }

        public static void Error(int line, string message) {
            Report(line, "", message);
        }

        private static void Report(int line, string where, string message) {
            Console.Error.WriteLine("[" + line + "] Error" + where + ": " + message);
            hadError = true;
        }
    }
}