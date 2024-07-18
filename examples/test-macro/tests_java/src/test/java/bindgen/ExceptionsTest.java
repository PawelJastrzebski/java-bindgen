package bindgen;

import org.junit.jupiter.api.Test;

import com.test.macro.TestMacro;

import static org.junit.jupiter.api.Assertions.*;

public class ExceptionsTest {

    @Test
    public void should_throw_1() {
        assertThrows(RuntimeException.class, () -> {
            try {
                TestMacro.should_throw_exception_1(1);
            } catch (Exception e) {
                System.out.println(e + "");
                throw e;
            }
        });
    }

    @Test
    public void should_throw_2() {
        assertThrows(ArithmeticException.class, () -> {
            try {
                TestMacro.should_throw_exception_2(1);
            } catch (ArithmeticException e) {
                System.out.println(e.getMessage());
                assertTrue(e.getMessage().contains("[err_message_2]"));
                throw e;
            }
        });
    }

    @Test
    public void should_throw_3() {
        assertThrows(IllegalStateException.class, () -> {
            try {
                TestMacro.should_throw_exception_3(1);
            } catch (IllegalStateException e) {
                System.out.println(e.getMessage());
                assertTrue(e.getMessage().contains("IllegalStateException"));
                throw e;
            }
        });
    }

    @Test
    public void should_throw_4() {
        assertThrows(SecurityException.class, () -> {
            try {
                TestMacro.should_throw_exception_4(1);
            } catch (SecurityException e) {
                System.out.println(e.getMessage());
                assertTrue(e.toString().contains("SecurityException"));
                throw e;
            }
        });
    }

    @Test
    public void should_throw_exception_5() {
        assertThrows(RuntimeException.class, () -> {
            try {
                TestMacro.should_throw_exception_5(1);
            } catch (RuntimeException e) {
                System.out.println(e.getMessage());
                assertTrue(e.toString().contains("Rust"));
                assertTrue(e.toString().contains("Backtrace"));
                assertTrue(e.toString().contains("Always Throw"));
                throw e;
            }
        });
    }

    @Test
    public void should_throw_exception_in_order() {
        assertThrows(IndexOutOfBoundsException.class, () -> {
            try {
                TestMacro.should_throw_exception_in_order(1);
            } catch (IndexOutOfBoundsException e) {
                System.out.println(e + "");
                throw e;
            }
        });
    }

}
