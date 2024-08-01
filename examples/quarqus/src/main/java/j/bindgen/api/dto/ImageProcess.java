package j.bindgen.api.dto;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.NotNull;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.NoArgsConstructor;
import org.jboss.resteasy.reactive.RestForm;

@Builder
@NoArgsConstructor
@AllArgsConstructor
public class ImageProcess {
    @NotNull
    @RestForm
    public byte[] image;
    @NotBlank
    @RestForm
    public String imageExtension;
    @NotNull
    @RestForm
    public String transforms;
}
