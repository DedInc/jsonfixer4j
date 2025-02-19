package com.github.dedinc.jsonfixer4j;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Deque;
import java.util.List;

public final class JSONTokenFixer {

    public static List<Token> fixTokens(List<Token> tokens) {
        List<Token> fixed = new ArrayList<>(tokens.size());
        Deque<TokenKind> stack = new ArrayDeque<>(tokens.size());
        for (Token token : tokens) {
            switch (token.kind) {
                case LBRACE:
                case LBRACKET:
                    fixed.add(token);
                    stack.push(token.kind == TokenKind.LBRACE ? TokenKind.RBRACE : TokenKind.RBRACKET);
                    break;
                case RBRACE:
                case RBRACKET:
                    if (!stack.isEmpty() && token.kind == stack.peek()) {
                        stack.pop();
                        fixed.add(token);
                    } else {
                        TokenKind expectedToken = stack.isEmpty() ? null : stack.peek();
                        fixed.add(createMatchingToken(expectedToken, token));
                        if (!stack.isEmpty() && token.kind == stack.peek()) {
                            stack.pop();
                            fixed.add(token);
                        }
                    }
                    break;
                default:
                    fixed.add(token);
                    break;
            }
        }
        while (!stack.isEmpty()) {
            TokenKind expectedToken = stack.pop();
            fixed.add(createMatchingToken(expectedToken, null));
        }
        return fixed;
    }

    private static Token createMatchingToken(TokenKind expectedToken, Token originalToken) {
        if (expectedToken == TokenKind.RBRACE) {
            return new Token(TokenKind.RBRACE, "}");
        } else if (expectedToken == TokenKind.RBRACKET) {
            return new Token(TokenKind.RBRACKET, "]");
        }
        return originalToken;
    }
}