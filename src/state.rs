trait State {
    pub fn Draw();
    pub fn Update();
}

struct MainMenu {
    
}

impl State for MainMenu {
    pub fn Draw(){
        Ok(())
    }
    pub fn Update(){
        Ok(())
    }
}