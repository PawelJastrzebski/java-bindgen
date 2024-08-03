package bindgen;

import com.test.macro.*;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

public class ReturnOptionalTest {

    @Test
    public void return_int_optional() {
        int some = TestMacro.return_int_optional_some();
        assertEquals(10, some);

        // no null representation for primitive
        int none = TestMacro.return_int_optional_none();
        assertEquals(0, none);
    }

    @Test
    public void return_JInt_optional() {
        int some = TestMacro.return_JInt_optional_some();
        assertEquals(10, some);

        Integer none = TestMacro.return_JInt_optional_none();
        assertNull(none);
    }

    @Test
    public void return_void_optional() {
        TestMacro.return_void_optional_some();
        TestMacro.return_void_optional_none();
    }

    @Test
    public void return_bool_optional() {
        boolean some = TestMacro.return_bool_optional_some();
        assertTrue(some);

        // no null representation for primitive
        boolean none = TestMacro.return_bool_optional_none();
        assertFalse(none);
    }

    @Test
    public void return_char_optional() {
        char some = TestMacro.return_char_optional_some();
        assertEquals('j', some);

        // no null representation for primitive
        char none = TestMacro.return_char_optional_none();
        assertEquals('\0', none);
    }

}
