std::io::{BufReader, Write, Read, BufRead};

struct IOEncoder<E, I, O> {
    encoder: E,
    input: I,
    output: O,
}


impl<E, I, O> IOEncoder<E, I, O> 
    where
        E: Encoder + Decoder,
        I: Read,
        O: Write,
{
    
    /// Creates a new IOEncoder
    fn new(encoder: E, input: I, output: O) -> Self { 
        Self { 
            encoder, 
            input, 
            output 
        }
    }

    /// Process this encoders input by encoding each line of the input and saving the encoded version to
    /// the ouput
    fn encode(&mut self) -> Result<(), std::io::Error> {
        let reader = BufReader::new(&mut self.input);

        for line in reader.lines() {
            let line = line?;
            let encoded_line = self.encoder.encode(&line);
            writeln!(self.output, "{}", encoded_line)?;
        }
        Ok(())
    }

    /// Process this encoders input by decoding each line of the input and saving the decoded version to
    /// the ouput
    fn decode(&mut self) -> Result<(), std::io::Error> { 
        let reader = BufReader::new(&mut self.input);

        for line in reader.lines() {
            let line = line?;
            let decoded_line = self.encoder.decode(&line);
            writeln!(self.output, "{}", decoded_line)?;
        }
        Ok(())
    }

}