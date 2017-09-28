use std::collections::HashMap;


fn main() {

    let my_string = String::from("hello world world");
    let con1 = concordance(&my_string);

    println!("{:?}", con1);
    println!("Hello, world!");

}

fn concordance(document: &str) -> HashMap<&str, u32> {
    let mut concordance = HashMap::new();
    for word in document.split_whitespace() {
        let count = concordance.entry(word).or_insert(0);
        *count += 1;
    }

    concordance
}

/*
class VectorCompare:
    def magnitude(self, concordance):
        total = 0
        for word, count in concordance.iteritems():
            total += count ** 2
        return math.sqrt(total)

    def relation(self, concordance1, concordance2):
        topvalue = 0
        for word, count in concordance1.iteritems():
            if word in concordance2:
                topvalue += count * concordance2[word]
        if (self.magnitude(concordance1) * self.magnitude(concordance2)) != 0:
            return topvalue / (self.magnitude(concordance1) * self.magnitude(concordance2))
        else:
            return 0

    def concordance(self, document):
        con = {}
        for word in document.split(' '):
            if word in con:
                con[word] = con[word] + 1
            else:
                con[word] = 1
        return con
        */