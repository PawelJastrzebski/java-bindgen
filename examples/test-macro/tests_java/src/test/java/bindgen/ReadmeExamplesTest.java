package bindgen;

import org.junit.jupiter.api.Test;
import com.test.macro.*;

import java.util.LinkedList;

import static org.junit.jupiter.api.Assertions.*;

public class ReadmeExamplesTest {

    @Test
    public void complex_types() {
        Node parent = new Node(1);
        Node child = new Node(2);
        Element element = Element.builder().children(new LinkedList<>()).parent(parent).build();

        Element updated = TestMacro.add_new_node(child, element);
        System.out.println("Updated: " + updated);

        assertEquals(1, updated.getChildren().size());
    }
    
}
