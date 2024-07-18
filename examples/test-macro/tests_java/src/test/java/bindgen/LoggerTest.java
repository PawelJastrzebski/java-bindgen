package bindgen;

import org.junit.jupiter.api.Test;

import com.test.macro.TestMacro;

import static org.junit.jupiter.api.Assertions.*;

public class LoggerTest {

    @Test
    public void test_logger() {
        assertDoesNotThrow(() -> {
            TestMacro.test_logger("Java Bindgen");
        });
    }

}
