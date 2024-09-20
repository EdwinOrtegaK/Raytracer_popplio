mod framebuffer;
mod raytracer;
mod ray_intersect;

use framebuffer::Framebuffer;
use nalgebra::Vector3;
use ray_intersect::Material;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(0x000000); // Fondo negro
    framebuffer.clear();

    // Definimos los materiales
    let blue = Material { diffuse: 0x0000FF };  // Color azul
    let white = Material { diffuse: 0xFFFFFF };  // Color blanco
    let black = Material { diffuse: 0x000000 };  // Color negro
    let pink = Material { diffuse: 0xFF69B4 };   // Color rosado
    let dark_pink = Material { diffuse: 0xC71585 }; // Color rosado oscuro

    // Creamos la esfera azul que será la cabeza de Popplio
    // Añadimos las esferas blancas para los ojos, negras para las pupilas, y azules para las orejas
    let objects = vec![
        // Cabeza azul
        raytracer::Sphere {
            center: Vector3::new(0.0, 0.0, -5.0),
            radius: 1.5,
            material: blue,
        },
        // Ojo izquierdo (desde la perspectiva del observador)
        raytracer::Sphere {
            center: Vector3::new(-0.5, 0.0, -4.2),
            radius: 0.7,
            material: white,
        },
        // Ojo derecho (desde la perspectiva del observador)
        raytracer::Sphere {
            center: Vector3::new(0.5, 0.0, -4.2),
            radius: 0.7,
            material: white,
        },
        // Pupila izquierda (parte superior)
        raytracer::Sphere {
            center: Vector3::new(-0.55, 0.04, -3.0),
            radius: 0.25,
            material: black,
        },
        // Pupila izquierda (parte inferior)
        raytracer::Sphere {
            center: Vector3::new(-0.55, -0.04, -3.0),
            radius: 0.25,
            material: black,
        },
        // Pupila derecha (parte superior)
        raytracer::Sphere {
            center: Vector3::new(0.55, 0.04, -3.0),
            radius: 0.25,
            material: black,
        },
        // Pupila derecha (parte inferior)
        raytracer::Sphere {
            center: Vector3::new(0.55, -0.04, -3.0),
            radius: 0.25,
            material: black,
        },
        // Pupilas blancas adicionales (izquierda parte superior)
        raytracer::Sphere {
            center: Vector3::new(-0.55, 0.1, -2.8), // Posicionadas más cerca
            radius: 0.1,
            material: white,
        },
        // Pupilas blancas adicionales (izquierda parte inferior)
        raytracer::Sphere {
            center: Vector3::new(-0.55, 0.06, -2.8), // Posicionadas más cerca
            radius: 0.1,
            material: white,
        },
        // Pupilas blancas adicionales (derecha parte superior)
        raytracer::Sphere {
            center: Vector3::new(0.55, 0.1, -2.8), // Posicionadas más cerca
            radius: 0.1,
            material: white,
        },
        // Pupilas blancas adicionales (derecha parte inferior)
        raytracer::Sphere {
            center: Vector3::new(0.55, 0.06, -2.8), // Posicionadas más cerca
            radius: 0.1,
            material: white,
        },
        // Oreja izquierda
        raytracer::Sphere {
            center: Vector3::new(-1.8, 0.0, -5.0), // Posicionada a la izquierda de la cabeza
            radius: 0.5,
            material: blue,
        },
        // Oreja derecha
        raytracer::Sphere {
            center: Vector3::new(1.8, 0.0, -5.0), // Posicionada a la derecha de la cabeza
            radius: 0.5,
            material: blue,
        },
        // Nariz - Círculo más grande (parte inferior)
        raytracer::Sphere {
            center: Vector3::new(0.0, -0.5, -3.5), // Bajado y centrado entre los ojos
            radius: 0.35,
            material: white,
        },
        // Nariz - Círculo mediano (parte media)
        raytracer::Sphere {
            center: Vector3::new(0.0, -0.2, -3.3), // Un poco más abajo
            radius: 0.25,
            material: white,
        },
        // Nariz - Círculo pequeño (parte superior)
        raytracer::Sphere {
            center: Vector3::new(0.0, 0.0, -3.1), // Más abajo y más pequeño
            radius: 0.15,
            material: white,
        },
        // Nariz rosada
        raytracer::Sphere {
            center: Vector3::new(0.0, 0.15, -3.05), // Posición más arriba y ligeramente más adelante
            radius: 0.2,
            material: pink,
        },
        // Boca rosada oscura
        raytracer::Sphere {
            center: Vector3::new(0.0, -0.55, -3.4), // Posicionada entre los círculos de la nariz
            radius: 0.25,
            material: dark_pink,
        },
    ];

    // Renderizamos la escena
    raytracer::render(&mut framebuffer, &objects);

    // Mostrar el framebuffer en una ventana emergente
    framebuffer.display();
}
