SELECT ai.create_vectorizer( 
    'testimonials'::regclass,
    destination => 'testimonial_content_embeddings',
   embedding => jsonb_build_object(
        'function', 'ai.ollama_embed',
        'model', 'llama3',
        'input_text', 'content'  -- Automatically map the 'content' column
    ),
    chunking => ai.chunking_recursive_character_text_splitter('content')
);
