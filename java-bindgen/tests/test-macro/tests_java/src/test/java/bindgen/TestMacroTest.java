package bindgen;

import org.junit.jupiter.api.Test;

import com.test.macro.TestMacro;
import com.test.macro.UserClass;

import static org.junit.jupiter.api.Assertions.*;

public class TestMacroTest {

    @Test
    public void should_get_lib_metadata() {
        assertNotNull(TestMacro.libName);
        assertNotNull(TestMacro.libVersion);
        assertNotNull(TestMacro.libRelease);

        assertNotNull(TestMacro.libPath);
        assertNotNull(TestMacro.libExtension);
        System.out.println("Release date: " + TestMacro.libRelease);
    }

    @Test
    public void should_not_load_multiple_times() {
        TestMacro.loadNativeLibrary();
        TestMacro.loadNativeLibrary();
        TestMacro.loadNativeLibrary();
    }

    @Test
    public void returns_void() {
        TestMacro.returns_void();
    }

    @Test
    public void returns_jshort() {
        short s = TestMacro.returns_jshort();
        assertEquals(16, s);
    }

    @Test
    public void returns_jshort_i16() {
        short s = TestMacro.returns_jshort_i16();
        assertEquals(17, s);
    }

    @Test
    public void returns_jint() {
        int i = TestMacro.returns_jint();
        assertEquals(32, i);
    }

    @Test
    public void returns_jint_i32() {
        int i = TestMacro.returns_jint_i32();
        assertEquals(32, i);
    }

    @Test
    public void returns_jlong() {
        long l = TestMacro.returns_jlong();
        assertEquals(64L, l);
    }

    @Test
    public void returns_jlong_i64() {
        long l = TestMacro.returns_jlong_i64();
        assertEquals(64L, l);
    }

    @Test
    public void returns_jbyte() {
        byte b = TestMacro.returns_jbyte();
        assertEquals(8, b);
    }

    @Test
    public void returns_jbyte_u8() {
        byte b = TestMacro.returns_jbyte_u8();
        assertEquals(8, b);
    }

    @Test
    public void returns_jfloat() {
        float f = TestMacro.returns_jfloat();
        assertEquals(32.0f, f, 0.0);
    }

    @Test
    public void returns_jfloat_f32() {
        float f = TestMacro.returns_jfloat_f32();
        assertEquals(32.0f, f, 0.0);
    }

    @Test
    public void returns_jdouble() {
        double d = TestMacro.returns_jdouble();
        assertEquals(64.0, d, 0.0);
    }

    @Test
    public void returns_jdouble_i64() {
        double d = TestMacro.returns_jdouble_i64();
        assertEquals(64.0, d, 0.0);
    }

    @Test
    public void returns_boolean_bool() {
        boolean b = TestMacro.returns_boolean_bool();
        assertTrue(b);
    }    
    
    @Test
    public void returns_jchar() {
        char c = TestMacro.returns_jchar();
        assertEquals('y', c);
    }

    @Test
    public void returns_jchar_char() {
        char c = TestMacro.returns_jchar_char();
        assertEquals('y', c);
    }

    @Test
    public void returns_string() {
        String s = TestMacro.returns_string();
        assertEquals("ok string", s);
    }

    @Test
    public void returns_byte_array() {
        byte[] array = TestMacro.returns_byte_array();
        assertArrayEquals(new byte[] { 1, 2, 3 }, array);
    }

    @Test
    public void returns_JByte() {
        Byte b = TestMacro.returns_JByte();
        assertEquals(Byte.valueOf((byte)2), b);
    }

    @Test
    public void returns_JShort() {
        Short b = TestMacro.returns_JShort();
        assertEquals(Short.valueOf((short)3), b);
    }

    @Test
    public void returns_JInt() {
        Integer b = TestMacro.returns_JInt();
        assertEquals(Integer.valueOf(4), b);
    }

    @Test
    public void returns_JLong() {
        Long b = TestMacro.returns_JLong();
        assertEquals(Long.valueOf(4L), b);
    }

    @Test
    public void returns_JFloat() {
        Float b = TestMacro.returns_JFloat();
        assertEquals(Float.valueOf(5.0f), b);
    }

    @Test
    public void returns_JDouble() {
        Double b = TestMacro.returns_JDouble();
        assertEquals(Double.valueOf(6.0f), b);
    }

    @Test
    public void returns_JBoolean() {
        Boolean b = TestMacro.returns_JBoolean();
        assertEquals(Boolean.valueOf(true), b);
    }

    @Test
    public void returns_JChar() {
        Character b = TestMacro.returns_JChar();
        assertEquals(Character.valueOf('y'), b);
    }

    @Test
    public void input_u8() {
        TestMacro.input_u8((byte) 8);
    }

    @Test
    public void input_i16() {
        TestMacro.input_i16((short) 16);
    }

    @Test
    public void input_i32() {
        TestMacro.input_i32(32);
    }

    @Test
    public void input_i64() {
        TestMacro.input_i64(64L);
    }

    @Test
    public void input_f32() {
        TestMacro.input_f32(32.0f);
    }

    @Test
    public void input_f64() {
        TestMacro.input_f64(64.0);
    }

    @Test
    public void input_string() {
        TestMacro.input_string("test");
    }

    @Test
    public void input_byte_array() {
        TestMacro.input_byte_array(new byte[] { 1, 2, 3 });
    }

    @Test
    public void get_user() {
        UserClass user = TestMacro.get_user();
        assertEquals("Tom", user.getName());
    }

    @Test
    public void test_logger() {
        TestMacro.test_logger("Java Bindgen");
    }

}
