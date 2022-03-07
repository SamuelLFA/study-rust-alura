fn main() {
    condicionais();
    repeticoes();

    ownership();
    pattern_matching();
    errors();
}

fn condicionais() {
    let idade:u8 = 18;
    let responsavel_autorizou = true;
    let eh_maior = idade >= 18;

    if eh_maior {
        println!("Pode entrar na balada");
    } else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar com assinatura do responsável")
    } else {
        println!("Não pode entrar na balada")
    }

    let condicao = if eh_maior { "maior"} else { "menor" };

    println!("É {} de idade", condicao);

    let linguagem = "Kotlin";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido"
    };

    println!("O proposito da linguagem {} é {}", linguagem, proposito);
}

fn repeticoes() {
    let multiplicador:u8 = 5;

    let mut contador:u8 = 0;

    while contador < 10 {
        contador += 1;

        if contador == 5 {
            continue;
        }

        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;
    loop {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);

        if contador == 10 {
            break;
        }
    }

    for i in 1..11 {
        println!("{}", i);
    }
}

fn ownership() {
    let mut uma_string = String::from("Vinicius");
    rouba(&mut uma_string);

    println!("{}", uma_string);
}

fn rouba(string: &mut String) {
    string.push_str(" Dias");
    println!("{}", string);
}

fn pattern_matching() {
    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Um pouco",
            4..=10 => "Um bocado",
            _ if x % 2 == 0 => "Uma boa quantidade e par",
            _ => "Muito"
        });
    }
}

fn errors() {
    // panic!("Erro proposital");
    match resultado() {
        Ok(s) => println!("{}", s),
        Err(numero) => println!("{}", numero)
    };
}

fn resultado() -> Result<String, u8> {
    // Ok(String::from("Tudo deu certo"));
    Err(123)
}