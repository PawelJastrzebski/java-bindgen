package bindgen;

import com.test.macro.*;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import static org.junit.jupiter.api.Assertions.*;

public class CustomTypesTest {

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
    public void pass_user_list() {
        List<UserClass> list = Arrays.asList(
                new UserClass("Java", 2),
                new UserClass("Rust", 3));
        List<UserClass> users = TestMacro.pass_user_list("Hello, ", list);

        assertEquals("1Hello, ", users.get(0).getName());
        assertEquals(10, users.get(0).getAge());

        assertEquals("Java", users.get(1).getName());
        assertEquals(2, users.get(1).getAge());

        assertEquals("Rust", users.get(2).getName());
        assertEquals(3, users.get(2).getAge());
    }

    @Test
    public void pass_all_types() {
        byte java_b = (byte) -7;
        short java_s = 2;
        int java_i = 12;
        long java_l = 23L;
        float java_f = 30.0f;
        double java_d = 40.0;
        char java_c = 'a';
        boolean java_bool = true;
        String java_String = "ok";
        byte[] java_b_array = new byte[] { 4, 5, 6 };

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
                .java_barray(java_b_array)
                .build();

        {
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
            assertArrayEquals(java_b_array, res.getJava_barray());
        }
        {
            List<AllJavaTypes> input = Arrays.asList(all, all);
            List<AllJavaTypes> list_res = TestMacro.pass_all_types_list(input);
            System.out.println(list_res);

            assertEquals(java_b, list_res.get(0).getJava_b());
            assertEquals(java_s, list_res.get(0).getJava_s());
            assertEquals(java_i, list_res.get(0).getJava_i());
            assertEquals(java_l, list_res.get(0).getJava_l());
            assertEquals(java_f, list_res.get(0).getJava_f());
            assertEquals(java_d, list_res.get(0).getJava_d());
            assertEquals(java_c, list_res.get(0).getJava_c());
            assertEquals(java_bool, list_res.get(0).isJava_bool());
            assertEquals(java_String, list_res.get(0).getJava_string());
            assertArrayEquals(java_b_array, list_res.get(0).getJava_barray());
        }

    }

    @Test
    public void pass_java_class_wrappers() {
        Byte java_b = (byte) -7;
        Short java_s = 2;
        Integer java_i = 12;
        Long java_l = 23L;
        Float java_f = 30.0f;
        Double java_d = 40.0;
        Character java_c = 'a';
        Boolean java_bool = true;

        JavaClassWrappers all = JavaClassWrappers
                .builder()
                .java_b(java_b)
                .java_s(java_s)
                .java_i(java_i)
                .java_l(java_l)
                .java_f(java_f)
                .java_d(java_d)
                .java_c(java_c)
                .java_bool(java_bool)
                .build();

        {
            JavaClassWrappers res = TestMacro.pass_java_class_wrappers(all);
            System.out.println(res);

            assertEquals(java_b, res.getJava_b());
            assertEquals(java_s, res.getJava_s());
            assertEquals(java_i, res.getJava_i());
            assertEquals(java_l, res.getJava_l());
            assertEquals(java_f, res.getJava_f());
            assertEquals(java_d, res.getJava_d());
            assertEquals(java_c, res.getJava_c());
            assertEquals(java_bool, res.getJava_bool());
        }
        {

            List<JavaClassWrappers> input = Arrays.asList(all, all);
            List<JavaClassWrappers> list_res = TestMacro.pass_java_class_wrappers_list(input);
            System.out.println(list_res);

            assertEquals(java_b, list_res.get(0).getJava_b());
            assertEquals(java_s, list_res.get(0).getJava_s());
            assertEquals(java_i, list_res.get(0).getJava_i());
            assertEquals(java_l, list_res.get(0).getJava_l());
            assertEquals(java_f, list_res.get(0).getJava_f());
            assertEquals(java_d, list_res.get(0).getJava_d());
            assertEquals(java_c, list_res.get(0).getJava_c());
            assertEquals(java_bool, list_res.get(0).getJava_bool());
        }
    }

    @Test
    public void pass_java_class_embeded() {
        EmbededTypes input = EmbededTypes.builder()
                .parent(new EmbededNode(10))
                .children(Arrays.asList(new EmbededNode(120), new EmbededNode(310)))
                .build();

        EmbededTypes embeded_result = TestMacro.pass_java_class_embeded(input);
        System.out.println(embeded_result);
        assertEquals(10, embeded_result.getParent().getNode_id());
        assertEquals(120, embeded_result.getChildren().get(0).getNode_id());
        assertEquals(310, embeded_result.getChildren().get(1).getNode_id());
    }

    @Test
    public void pass_java_class_embeded_multi_thread() throws InterruptedException {
        List<CompletableFuture<Void>> list = new ArrayList<>();
        ExecutorService pool = Executors.newFixedThreadPool(4);
        for (int i = 0; i < 10_000; i++) {
            list.add(CompletableFuture.runAsync(() -> {

                Node parent = new Node(1);
                Node child = new Node(2);
                Element element = Element.builder().children(new LinkedList<>()).parent(parent).build();

                Element updated = TestMacro.add_new_node(child, element);
//                System.out.println("Updated: " + updated);

                assertEquals(1, updated.getChildren().size());

            }, pool));
        }

        for (CompletableFuture<Void> t : list) {
            t.join();
        }
    }

}
