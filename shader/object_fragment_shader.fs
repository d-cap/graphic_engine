#version 330 core
struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float shininess;
};

uniform Material material;

struct Light {
    vec3 position;
    
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

uniform Light light;

in vec3 FragPos;
in vec3 Normal;

out vec4 FragColor;

uniform vec3 cameraPos;

void main()
{
    vec3 ambient = light.ambient * material.ambient;

    vec3 normal = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPos);
    
    float diff= max(dot(normal, lightDir), 0.0);
    vec3 diffuse = light.diffuse * (diff * material.diffuse);

    vec3 viewDir = normalize(cameraPos - FragPos);

    float spec = pow(max(dot(reflect(-lightDir, normal), viewDir), 0.0), material.shininess);
    vec3 specular = light.specular * (spec * material.specular);

    vec3 result = ambient + diffuse + specular ;
    FragColor = vec4(result, 1.0);
}