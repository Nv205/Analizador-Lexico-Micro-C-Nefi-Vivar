# Pre-compilador-micro-c-Nefi-Vivar

Nombre: Nefi Andŕe Vivar Castañeda
Carne: 202425505
Curso: Automatas y Lenguajes
Proyecto Compilador micro C


**Descripcion:**

El proyecto actual es el diseño de un pre-compilador micro c para el curso de Automatas y Lenguajes, actualmente solo contiene las funciones implementadas en base a la tarea del diseño del pre-compilador para el 22/02/26. con todos los botones del documento de pre-compilador implementadas, pero solo las funcionales que pide el documento hasta el momento, como el boton dalir, guardar, abrir, etc...


**Tecnologias usadas:**

Lenguaje: Rust

1. eframe / egui

Es el framework principal de la aplicación.

    Propósito: Se utiliza para crear la interfaz gráfica de usuario (GUI).

    Relación con el PDF: Permite crear el TextBox1 para el código , el TextBox2 para los mensajes y la barra de menús con las opciones de "Archivo", "Editar", "Compilar" y "Ayuda".

    Funcionalidad clave: Maneja el ciclo de vida de la ventana y el renderizado inmediato de los elementos.

2. rfd (Rust Native File Dialogs)

    Propósito: Proporciona los cuadros de diálogo nativos del sistema operativo para interactuar con archivos.

    Relación con el PDF: Es esencial para cumplir con la función [Abrir] (punto 4) al buscar archivos con extensión *.C y la función [Guardar] (punto 5), abriendo el cuadro de diálogo para solicitar la ubicación si el archivo es nuevo.

3. std::fs (Librería estándar de Rust)

    Propósito: Manejo del sistema de archivos del sistema operativo.

    Relación con el PDF: Se utiliza para leer el contenido de los archivos cargados en el TextBox1 y para escribir o sobreescribir el texto cuando el usuario decide guardar sus cambios.

4. std::path::PathBuf

    Propósito: Gestión de rutas de archivos de forma segura entre diferentes sistemas operativos (Windows/Linux).

    Relación con el PDF: Permite almacenar la ubicación del archivo para cumplir con el requisito 10, donde se debe desplegar la ubicación y nombre del archivo en el título de la ventana.


**Instrucciones de Ejecución:**

1. Requisitos Previos

Antes de intentar ejecutar el código, asegúrate de tener instalado lo siguiente:

    Rust Toolchain: Debes tener instalado rustup, rustc y cargo. Si no lo tienes, puedes descargarlo desde rustup.rs.

    Dependencias del Sistema:

        En Windows: Generalmente no requiere pasos adicionales.

        En Linux: Podrías necesitar librerías de desarrollo para la interfaz gráfica (como libgtk-3-dev, libssl-dev, y libclang-dev).

2. Configuración del Proyecto (Cargo.toml)

Asegúrate de que tu archivo Cargo.toml incluya las dependencias necesarias que hemos discutido para que la interfaz gráfica y los diálogos de archivos funcionen:
Ini, TOML

[dependencies]
eframe = "0.22"
rfd = "0.10"

3. Pasos para la Ejecución en VS Code

Sigue este orden en la terminal integrada de VS Code (Ctrl + `):

    Limpieza y Verificación: Ejecuta el siguiente comando para verificar que no haya errores de sintaxis sin necesidad de compilar todo el proyecto:
    Bash

    cargo check

    Descarga de dependencias y Compilación: La primera vez que lo ejecutes, Cargo descargará todas las librerías. Usa el comando:
    Bash

    cargo build

    Ejecución del Pre-Compilador: Para lanzar la ventana de la aplicación, usa:
    Bash

    cargo run

4. Pruebas de Funcionamiento (Basado en la Hoja de Trabajo)

Una vez que la ventana "MicroC Compiler" esté abierta, verifica los siguientes puntos exigidos por la Universidad Mesoamericana:

    Nuevo: Haz clic en Archivo > Nuevo. El TextBox1 debe limpiarse y permitirte escribir inmediatamente.

    Abrir: Haz clic en Archivo > Abrir y selecciona un archivo con extensión .C. Verifica que el texto se cargue pero no se pueda editar inicialmente.

    Editar: Haz clic en el botón Editar para desbloquear el TextBox1 y modificar el código cargado.

    Guardar:

        Si es un archivo nuevo, debe aparecer el cuadro de diálogo para elegir ubicación.

        Si el archivo ya existía, debe sobreescribirse automáticamente.

    Título de Ventana: Observa que en la parte superior aparezca la ruta completa y el nombre del archivo (ej. MicroC Compiler - C:/Proyectos/test.c).

    Salir: Intenta cerrar la aplicación con cambios sin guardar. Debe aparecer la ventana emergente que creamos preguntando si deseas guardar o salir.


**Capturas de pantalla**

<img width="907" height="705" alt="imagen" src="https://github.com/user-attachments/assets/c860f6e5-2728-4259-a52d-0b583b175a2f" />

<img width="949" height="721" alt="imagen" src="https://github.com/user-attachments/assets/db742a96-bfcc-402f-a135-93d92887157b" />

<img width="893" height="700" alt="imagen" src="https://github.com/user-attachments/assets/97896db6-2f12-423a-b8dc-3fb342d8c313" />



**Video Demostrativo:**

https://www.youtube.com/watch?v=GyQ89cy_juM







