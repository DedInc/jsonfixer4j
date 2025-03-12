package com.github.dedinc.jsonfixer4j;

public final class JSONFixerRust {

    static {

        try {
            System.loadLibrary("jsonfixer_rust");
        } catch (UnsatisfiedLinkError e) {
            System.err.println("Failed to load the jsonfixer_rust native library: " + e.getMessage());
            throw e;
        }
    }

    public native String autocorrect(String json);

    public static String fix(String json) {
        return new JSONFixerRust().autocorrect(json);
    }
}