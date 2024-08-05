package bindgen;

import com.test.macro.*;
import org.junit.jupiter.api.Test;

import java.util.List;

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
    public void return_str_optional() {
        String some = TestMacro.return_str_optional_some();
        assertEquals("Hello", some);

        String none = TestMacro.return_str_optional_none();
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

    @Test
    public void return_int_result_optional() {
        int some = TestMacro.return_int_result_optional_some();
        assertEquals(101, some);

        // no null representation for primitive
        int none = TestMacro.return_int_result_optional_none();
        assertEquals(0, none);
    }

    @Test
    public void return_str_result_optional() {
        String some = TestMacro.return_str_result_optional_some();
        assertEquals("Option<Hello>", some);

        String none = TestMacro.return_str_result_optional_none();
        assertNull(none);
    }

    @Test
    public void return_list_result_optional() {
        List<String> some = TestMacro.return_list_result_optional_some();
        assertNotNull(some);
        assertEquals("Option<Hello>", some.get(0));

        List<String> none = TestMacro.return_list_result_optional_none();
        assertNull(none);
    }

}
