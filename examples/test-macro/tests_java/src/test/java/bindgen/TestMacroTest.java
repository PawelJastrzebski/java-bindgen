package bindgen;

import org.junit.jupiter.api.Test;

import com.test.macro.AllJavaTypes;
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
        assertEquals(Byte.valueOf((byte) 2), b);
    }

    @Test
    public void returns_JShort() {
        Short b = TestMacro.returns_JShort();
        assertEquals(Short.valueOf((short) 3), b);
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

    // Input types

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

    // Pass Types

    @Test
    public void pass_u8() {
        assertEquals((byte) 8, TestMacro.pass_u8((byte) 8));
    }

    @Test
    public void pass_i16() {
        assertEquals(16, TestMacro.pass_i16((short) 16));
    }

    @Test
    public void pass_i32() {
        assertEquals(32, TestMacro.pass_i32(32));
    }

    @Test
    public void pass_i64() {
        assertEquals(64L, TestMacro.pass_i64(64L));
    }

    @Test
    public void pass_f32() {
        assertEquals(32.0f, TestMacro.pass_f32(32.0f));
    }

    @Test
    public void pass_f64() {
        assertEquals(64.0, TestMacro.pass_f64(64.0));
    }

    @Test
    public void pass_string() {
        assertEquals("test", TestMacro.pass_string("test"));
    }

    @Test
    public void pass_byte_array() {
        assertArrayEquals(new byte[] { 1, 2, 3 }, TestMacro.pass_byte_array(new byte[] { 1, 2, 3 }));
    }

    @Test
    public void get_user() {
        UserClass user = TestMacro.get_user();
        assertEquals("Tom", user.getName());
    }

    @Test
    public void pass_user() {
        UserClass user = TestMacro.pass_user("Hello, ", new UserClass("Java", 2));
        System.out.println(user);
        assertEquals("Hello, Java", user.getName());
        assertEquals(102, user.getAge());
    }

    @Test
    public void pass_all_types() {
        byte java_b = (byte) -7;
        short java_s = 2;
        int java_i = 12;
        long java_l = 23l;
        float java_f = 30.0f;
        double java_d = 40.0;
        char java_c = 'a';
        boolean java_bool = true;
        String java_String = "ok";
        byte[] java_barray = new byte[] { 4, 5, 6 };

        AllJavaTypes all = AllJavaTypes
                .builder()
                .java_b(java_b)
                .java_s(java_s)
                .java_i(java_i)
                .java_l(java_l)
                .java_f(java_f)
                .java_d(java_d)
                .java_c(java_c)
                .java_bool(java_bool)
                .java_string(java_String)
                .java_barray(java_barray)
                .build();

        AllJavaTypes res = TestMacro.pass_all_types(all);
        System.out.println(res);

        assertEquals(java_b, res.getJava_b());
        assertEquals(java_s, res.getJava_s());
        assertEquals(java_i, res.getJava_i());
        assertEquals(java_l, res.getJava_l());
        assertEquals(java_f, res.getJava_f());
        assertEquals(java_d, res.getJava_d());
        assertEquals(java_c, res.getJava_c());
        assertEquals(java_bool, res.isJava_bool());
        assertEquals(java_String, res.getJava_string());
        assertArrayEquals(java_barray, res.getJava_barray());
    }

    @Test
    public void test_logger() {
        TestMacro.test_logger("Java Bindgen");
    }

}
