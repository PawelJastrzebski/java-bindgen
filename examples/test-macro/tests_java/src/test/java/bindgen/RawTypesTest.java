package bindgen;

import org.junit.jupiter.api.Test;

import com.test.macro.TestMacro;

import static org.junit.jupiter.api.Assertions.*;

public class RawTypesTest {

    @Test
    public void should_pass_raw_types_dirrerent_rust_args_order() {
        assertEquals("1", TestMacro.test_raw_types_1("1"));
        assertEquals("2", TestMacro.test_raw_types_2("2"));
        assertEquals("3", TestMacro.test_raw_types_3("3"));
        assertEquals("4", TestMacro.test_raw_types_4("4"));
        assertEquals("5", TestMacro.test_raw_types_5("5"));
        assertEquals("6", TestMacro.test_raw_types_6("6"));
        assertEquals("7", TestMacro.test_raw_types_7("7"));
    }

}
