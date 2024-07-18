package bindgen;

import com.test.macro.TestMacro;
import com.test.macro.UserClass;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class NullArgsTest {

    @Test
    public void null_method_arg() {
        assertThrows(NullPointerException.class, () -> {
            try {
                TestMacro.input_string(null);
            } catch (NullPointerException e) {
                System.out.println(e.getMessage());
                assertTrue(e.getMessage().contains("Null pointer"));
                throw e;
            }
        });
    }

    @Test
    public void null_method_arg_custom_type_1() {
        assertThrows(NullPointerException.class, () -> {
            try {
                TestMacro.pass_user("Hello, ", null);
            } catch (NullPointerException e) {
                System.out.println(e.getMessage());
                assertTrue(e.getMessage().contains("Null pointer"));
                throw e;
            }
        });
    }

    @Test
    public void null_method_arg_custom_type_2() {
        assertThrows(NullPointerException.class, () -> {
            try {
                TestMacro.pass_user("Hello, ", new UserClass(null, 2));
            } catch (NullPointerException e) {
                System.out.println(e.getMessage());
                assertTrue(e.getMessage().contains("Null pointer"));
                throw e;
            }
        });
    }

}
