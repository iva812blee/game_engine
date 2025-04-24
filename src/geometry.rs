// структура для представления точки в 3D пространстве
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x_: f64, y_: f64, z_: f64) -> Self {
        Self {
            x: x_,
            y: y_,
            z: z_,
        }
    }
}

// структура для представления вектора в 3D пространстве
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    // Создание вектора из двух точек
    // a - начальная точка, b - конечная точка */
    pub fn new(a: &Point, b: &Point) -> Self {
        Self {
            x: b.x - a.x,
            y: b.y - a.y,
            z: b.z - a.z,
        }
    }

    // Вычисление длины вектора
    // длина вектора = sqrt(x^2 + y^2 + z^2)
    // где x, y, z - координаты вектора
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Вычисление скалярного произведения двух векторов
    pub fn scalar_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Вычисление векторного произведения двух векторов
    pub fn cross_product(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // Вычисление угла между двумя векторами
    // угол = arccos((a * b) / (|a| * |b|))
    pub fn angle_between(&self, other: &Self) -> f64 {
        let dot_product = self.scalar_product(other);
        let length_a = self.length();
        let length_b = other.length();
        (dot_product / (length_a * length_b)).acos()
    }
}

// Вычисление расстояния между двумя точками
// расстояние = sqrt((x2 - x1)^2 + (y2 - y1)^2 + (z2 - z1)^2)
pub fn distance(a: &Point, b: &Point) -> f64 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2) + (b.z - a.z).powi(2)).sqrt()
}

// ПОВОРОТ ВЕКТОРА
// угол - угол поворота в радианах
// v - вектор, который нужно повернуть
// возвращает новый вектор, который является результатом поворота
// поворот вектор вокруг оси Z
pub fn rotate_vector_z(angle: f64, v: &Vector) -> Vector {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    Vector {
        x: v.x * cos_angle - v.y * sin_angle,
        y: v.x * sin_angle + v.y * cos_angle,
        z: v.z,
    }
}

// Поворот вектора вокруг оси Y
pub fn rotate_vector_y(angle: f64, v: &Vector) -> Vector {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    Vector {
        x: v.x * cos_angle + v.z * sin_angle,
        y: v.y,
        z: -v.x * sin_angle + v.z * cos_angle,
    }
}

// Поворот вектора вокруг оси X
pub fn rotate_vector_x(angle: f64, v: &Vector) -> Vector {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    Vector {
        x: v.x,
        y: v.y * cos_angle - v.z * sin_angle,
        z: v.y * sin_angle + v.z * cos_angle,
    }
}