import com.github.dedinc.jsonfixer4j.JSONFixerRust;

public class Main {
    public static void main(String[] args) {
        JSONFixerRust corrector = new JSONFixerRust();

        String[] brokenCases = {
                "{\"key\": 123",                    // Missing closing brace
                "{{\"name\": \"Test\"}",            // Extra brace at the start
                "{\"arr\": [1, 2, 3}",              // Missing closing bracket for array
                "{\"key\": \"test\", \"star, ",     // Unfinished key
                "{\"key\": \"test\", \"new\": fals",// Incomplete boolean
                "{\"title\": \"Hello",              // Unterminated string
                "{\"key1\": 1, \"key2\": 2,",       // Trailing comma
                "{\"one\": 1 \"two\": 2}",          // Missing comma
                "{\"flag\": tr, \"value\": nul}"    // Partial literals
        };

        for (String broken : brokenCases) {
            String fixed = corrector.autocorrect(broken);
            System.out.println("Broken:  " + broken);
            System.out.println("Fixed:   " + fixed);
            System.out.println("----------------------");
        }
    }
}
