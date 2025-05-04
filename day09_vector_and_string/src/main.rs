
fn main() {
	     let mut names = vec!["Alice".to_string(), "Bob".to_string(), "Carol".to_string()];

	            // Agregar un nuevo nombre
	                names.push("Dave".to_string());

	                    // Mostrar todos los nombres
	                        println!("List of names:");
	                            for (i, name) in names.iter().enumerate() {
	                            	        println!("{}: {}", i + 1, name);
	                            	            }

	                            	                // Buscar un nombre específico
	                            	                    let target = "Carol";
	                            	                        if names.contains(&target.to_string()) {
	                            	                        	        println!("Found {}", target);
	                            	                        	            } else {
	                            	                        	            	        println!("{} not found", target);
	                            	                        	            	            }

	                            	                        	            	                // Reemplazar un nombre
	                            	                        	            	                    if let Some(pos) = names.iter().position(|x| x == "Bob") {
	                            	                        	            	                    	        names[pos] = "Bobby".to_string();
	                            	                        	            	                    	            }

	                            	                        	            	                    	                // Concatenar todos los nombres en un solo string
	                            	                        	            	                    	                    let joined = names.join(", ");
	                            	                        	            	                    	                        println!("All names: {}", joined);
	                            	                        	            	                    	                        }
	                            	                        	            	                    }
	                            	                        	            }
	                            	                        }
	                            }
}

