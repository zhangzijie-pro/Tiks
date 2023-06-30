use embed_resource;

// there is a problem our command can't add logo
pub fn logo_to_exe(){
    embed_resource::compile(r"C:\Users\lenovo\Desktop\rust\zip_code\code_to_zip\logo.rc", embed_resource::NONE);
}