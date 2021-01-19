/// Banner text art developed using: https://manytools.org/hacker-tools/ascii-banner/
pub fn get_banner() -> String {
    format!(
        "{cyan}
   oooo  .oooo.o  .ooooo.  ooo. .oo.    .ooooo.  oooo    ooo 
   `888 d88(  \"8 d88' `88b `888P\"Y88b  d88' `88b  `88b..8P'  
    888 `\"Y88b.  888   888  888   888  888   888    Y888'    
    888 o.  )88b 888   888  888   888  888   888  .o8\"'88b   
    888 8\"\"888P' `Y8bod8P' o888o o888o `Y8bod8P' o88'   888o 
    888                                     {purple}BY: NILAY SAVANT{nc}{cyan} 
.o. 88P                                                      
`Y888P{nc}                                                       
",
        cyan = "\x1b[0;36m",
        purple = "\x1b[0;35m",
        nc = "\x1b[0m",
    )
}
