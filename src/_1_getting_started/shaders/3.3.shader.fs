
        #version 420 core
        out vec4 FragColor;

        in vec3 ourColor;
				uniform float rCol;

        void main() 
        {
            FragColor = vec4(rCol, ourColor.gb, 1.0); 
        }
