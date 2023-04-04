use std::fs;
use three_d::*;
use regex::Regex;
use rfd::FileDialog;




use pest::{Parser, iterators::Pairs};

#[derive(Parser)]
#[grammar = "jbeam.pest"]
pub struct JBeamParser;

fn parse_node_modifiers(rule: Pairs<Rule>, node: JNode) -> JNode {

    let mut node = node.clone();

    

    for rule in rule {
        for rule in rule.into_inner() {
            // these are the actual modifier rules
            // println!("{:?}", rule.as_rule());

            match rule.as_rule() {
                Rule::node_weight => {

                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.node_weight = rule.as_str().parse().unwrap();
                        }
                    }

                },
                Rule::node_collision => {


                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::boolean {
                            node.collision = rule.as_str().parse().unwrap();
                        }
                    }


                },
                Rule::node_self_collision => {


                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::boolean {
                            node.self_collision = rule.as_str().parse().unwrap();
                        }
                    }

                },
                Rule::node_group => {
                    let mut node_groups: Vec<String> = Vec::new();

                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            node_groups.push(rule.as_str().to_string().replace(r#"""#, ""));
                        }
                    }
                    node.group.append(&mut node_groups);
                },
                Rule::node_friction_coef => {

                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.friction_coefficient = rule.as_str().parse().unwrap();
                        }
                    }

                },
                Rule::node_material => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            node.node_material = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::node_fixed => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::boolean {
                            node.fixed = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_coupler_strength => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.coupler_strength = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_coupler_tag => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.coupler_tag = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_coupler_radius => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.coupler_radius = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_break_group => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            node.break_group = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::node_coupler_lock => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::boolean {
                            node.coupler_lock = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_import_electrics => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            node.import_electrics.push(rule.as_str().to_string().replace(r#"""#, ""));
                        }
                    }
                },
                Rule::node_import_inputs => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            node.import_inputs.push(rule.as_str().to_string().replace(r#"""#, ""));
                        }
                    }
                },
                Rule::node_surface_coef => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.surface_coef = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_volume_coef => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.volume_coef = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_no_load_coef => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.no_load_coef = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_full_load_coef => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.full_load_coef = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_stribeck_exponent => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.stribeck_exponent = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_stribeck_vel_mult => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.stribeck_vel_mult = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_softness_coef => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.softness_coef = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_tread_coef => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.tread_coef = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_tag => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            node.tag = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::node_load_sensitivity_slope => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.load_sensitivity_slope = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_paired_node => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            node.paired_node = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::node_chem_energy => {

                },
                Rule::node_burn_rate => {

                },
                Rule::node_flash_point => {

                },
                Rule::node_spec_heat => {

                },
                Rule::node_smoke_point => {

                },
                Rule::node_self_ignition_coef => {

                },
                Rule::node_engine_group => {

                },
                _=> ()
            }

        }
    }

    println!("{:#?}", node);

    return node;

    
}

pub fn parse_nodes(unparsed_file: String) -> Vec<JNode>{
    
    
    let file = JBeamParser::parse(Rule::parts, &unparsed_file).expect("Failed to parse JBeam!").next().unwrap();

    let mut nodes: Vec<JNode> = Vec::new();

    let mut node = JNode::new();

    for rule in file.into_inner() {
        if rule.as_rule() == Rule::nodes {


            

            for rule in rule.into_inner() {
                match rule.as_rule() {
                    Rule::node => {


                        let mut coord_counter = 0;
                        for rule in rule.into_inner() {
                            match rule.as_rule() {
                                Rule::string => {
                                    node.id = rule.as_str().to_string().replace(r#"""#, "");

                                },
                                Rule::number => {
                                    let value: f32 = rule.as_str().parse().unwrap();
                                    match coord_counter {
                                        0 => {
                                            node.position.0 = value;
                                        },
                                        1 => {
                                            node.position.2 = value;
                                        },
                                        2 => {
                                            node.position.1 = value;
                                        },
                                        _=> ()
                                    }
                                    coord_counter += 1;
                                },
                                Rule::node_modifiers => {
                                    node = parse_node_modifiers(rule.into_inner(), node)
                                }
                                _=> ()
                            }
                        }

                        // println!("{}, {:?}", id, coords);

                        nodes.push(node.clone());
                        node = JNode::new();

                    },
                    Rule::node_modifiers => {
                        node = parse_node_modifiers(rule.into_inner(), node);
                    },
                    _ => ()
                }
            }
        }
    }

    nodes
}







#[derive(Debug, Clone)]
pub struct JNode {
    pub id: String,
    pub position: (f32, f32, f32),
    pub imported: bool,
    // optional arguments
    pub node_weight: f64,
    collision: bool,
    self_collision: bool,
    pub group: Vec<String>,
    friction_coefficient: f32,
    node_material: String,
    fixed: bool,
    coupler_strength: f32,
    coupler_tag: f32,
    coupler_radius: f32,
    break_group: String,
    coupler_lock: bool,
    import_electrics: Vec<String>,
    import_inputs: Vec<String>,
    surface_coef: f32,
    volume_coef: f32,
    no_load_coef: f32,
    full_load_coef: f32,
    stribeck_exponent: f32,
    stribeck_vel_mult: f32,
    softness_coef: f32,
    tread_coef: f32,
    tag: String,
    load_sensitivity_slope: f32,
    paired_node: String,

    // editor specific thing... these should NOT get exported to BeamNG
    pub is_selected: bool,


}
impl JNode {
    pub fn new() -> Self {
        Self {
            id: "".to_owned(),
            position: (0.0, 0.0, 0.0),
            imported: true,
            node_weight: 0.0,
            collision: false,
            self_collision: false,
            group: Vec::new(),
            friction_coefficient: 1.0,
            node_material: "Metal".to_owned(),
            fixed: false,
            // highest f32 value
            coupler_strength: 340282346638528859811704183484516925440.0,
            coupler_tag: 1.0,
            coupler_radius: 0.0,
            break_group: String::new(),
            coupler_lock: false,
            import_electrics: Vec::new(),
            import_inputs: Vec::new(),
            surface_coef: 0.1,
            volume_coef: 0.1,
            no_load_coef: 1.0,
            full_load_coef: 0.0,
            stribeck_exponent: 1.75,
            stribeck_vel_mult: 1.0,
            softness_coef: 0.5,
            tread_coef: 0.5,
            tag: String::new(),
            load_sensitivity_slope: 0.0,
            paired_node: String::new(),

            is_selected: false,
        }
    }


    pub fn write(&self, filename: String) -> String{
        let group_str = if self.group.len() > 0 {
            format!(r#""group":["{}"]"#, self.group.join("\",\""))
        } else {
            String::new()
        };
        let data = format!(r#"["{}", {}, {}, {}, {{"nodeWeight":{}, "collision":{}, "selfCollision":{}, "frictionCoef":{}, {}}}],"#, self.id, self.position.0, self.position.2, self.position.1, self.node_weight, self.collision, self.self_collision, self.friction_coefficient, group_str);
        println!("{}", data);

        data

    }

    pub fn get_3d_object(&self, context: &Context, unselected_material: &CpuMaterial, selected_material: &CpuMaterial) -> Gm<Mesh, PhysicalMaterial> {
        if self.is_selected {
            let mut sphere = Gm::new(
                Mesh::new(&context, &CpuMesh::sphere(4)),
                PhysicalMaterial::new_opaque(
                    &context,
                    selected_material,
                ),
            );
            sphere.set_transformation(Mat4::from_translation(vec3(self.position.0, self.position.1, self.position.2)) * Mat4::from_scale(0.02));
            return  sphere;
        } else {
            let mut sphere = Gm::new(
                Mesh::new(&context, &CpuMesh::sphere(4)),
                PhysicalMaterial::new_opaque(
                    &context,
                    unselected_material,
                ),
            );
            sphere.set_transformation(Mat4::from_translation(vec3(self.position.0, self.position.1, self.position.2)) * Mat4::from_scale(0.02));
            return sphere;
        }



    }


}

#[derive(Debug, Clone)]
pub enum BeamType {
    Normal,
    Hydro,
    ANISOTROPIC,
    BOUNDED,
    PRESSURED,
    SUPPORT,
    BROKEN,
    LBEAM,
}

#[derive(Debug, Clone)]
pub struct JBeam {
    pub id1: String,
    pub id2: String,
    pub imported: bool,
    // optional arguments
    beam_type: String,
    beam_spring: f32,
    beam_damp: f32,
    beam_strength: f32,
    beam_deform: f32,
    beam_compression: f32,
    beam_compression_range: f32,
    beam_compression_time: f32,
    break_group: String,
    break_group_type: String,
    name: String,
    damp_cutoff_hz: f32,
    deform_limit: f32,
    deform_limt_expansion: f32,
    optional: bool,
    deform_group: String,
    deformation_trigger_ratio: f32,
    sound_file: String,
    color_factor: f32,
    attack_factor: f32,
    volume_factor: f32,
    decay_factor: f32,
    pitch_factor: f32,
    max_stress: f32,

}

impl JBeam {
    pub fn new() -> Self {
        Self {
            id1: String::new(),
            id2: String::new(),
            imported: true,
            beam_type: "Normal".to_owned(),
            beam_spring: 0.0,
            beam_damp: 0.0,
            beam_strength: 0.0,
            beam_deform: 0.0,
            beam_compression: 0.0,
            beam_compression_range: 0.0,
            beam_compression_time: 0.0,
            break_group: String::new(),
            break_group_type: "0".to_owned(),
            name: String::new(),
            damp_cutoff_hz: 0.0,
            deform_limit: 0.0,
            deform_limt_expansion: 0.0,
            optional: false,
            deform_group: String::new(),
            deformation_trigger_ratio: 0.0,
            sound_file: String::new(),
            color_factor: 0.0,
            attack_factor: 0.0,
            volume_factor: 0.0,
            decay_factor: 0.0,
            pitch_factor: 0.0,
            max_stress: 0.0,
        }
    }
    pub fn write(&self) -> String {

        let data = format!(r#"["{}", "{}" {{"beamType":{}, "beamSpring":{}, "beamDamp":{}, "beamStrength":{}, "beamDeform":{}, "beamPrecompression":{}, "beamPrecompressionRange":{}, "beamPrecompressionTime":{}, "breakGroup":{}, "breakGroupType":{}, "name":{}, "dampCutoffHz":{}, "deformLimit":{}, "deformLimitExpansion":{}, "optional":{}, "deformGroup":{}, "deformationTriggerRatio":{}, "soundFile":{}, "colorFactor":{}, "attackFactor":{}, "volumeFactor":{}, "decayFactor":{}, "pitchFactor":{}, "maxStress":{} }}],"#,
            self.id1,
            self.id2,
            self.beam_type,
            self.beam_spring,
            self.beam_damp,
            self.beam_strength,
            self.beam_deform,
            self.beam_compression,
            self.beam_compression_range,
            self.beam_compression_time,
            self.break_group,
            self.break_group_type,
            self.name,
            self.damp_cutoff_hz,
            self.deform_limit,
            self.deform_limt_expansion,
            self.optional,
            self.deform_group,
            self.deformation_trigger_ratio,
            self.sound_file,
            self.color_factor,
            self.attack_factor,
            self.volume_factor,
            self.decay_factor,
            self.pitch_factor,
            self.max_stress,
        );
        println!("{}", data);
        data

    }

    pub fn get_3d_object(&self, context: &Context, nodes: &Vec<JNode>) -> Sprites {
        let pos1 = nodes[get_node_by_id(self.id1.clone(), nodes).unwrap()].position;
        let pos2 = nodes[get_node_by_id(self.id2.clone(), nodes).unwrap()].position;



        let (x, y, z) = get_midpoint(pos1.0, pos2.0, pos1.1, pos2.1, pos1.2, pos2.2);



        let mut sprites = Sprites::new(
            context,
            &[vec3(x, y, z)],
            None,
        );

        let new_direction = vec3(pos2.0 - pos1.0, pos2.1 - pos1.1, pos2.2 - pos1.2);

        sprites.set_direction(Some(new_direction));

        // set the scale based on the new_direction

        

        let new_transform = Mat4::from_nonuniform_scale(0.0025, 0.5, 0.0025);
        sprites.set_transformation(new_transform);

        sprites
        
        
    }

}


fn get_midpoint(x1: f32, x2: f32, y1: f32, y2: f32, z1: f32, z2: f32) -> (f32, f32, f32) {
    let x = (x1 + x2) / 2.0;
    let y = (y1 + y2) / 2.0;
    let z = (z1 + z2) / 2.0;

    (x, y, z)
}


pub fn get_node_by_id(id: String, nodes: &Vec<JNode>) -> Option<usize> {
    // get the index of the node with the given id
    for (i, node) in nodes.iter().enumerate() {
        if node.id == id {
            return Some(i);
        }
    }
    None
}








fn parse_beam_modifiers(rule: Pairs<Rule>, beam: JBeam) -> JBeam {

    let mut beam = beam.clone();

    for rule in rule {
        for rule in rule.into_inner() {
            // these are the actual beam modifiers
            match rule.as_rule() {
                Rule::beam_beam_type => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            beam.beam_type = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::beam_beam_spring => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.beam_spring = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_beam_damp => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.beam_damp = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_beam_strength => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.beam_strength = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_beam_deform => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.beam_deform = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_beam_precompression => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.beam_compression = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_beam_precompression_range => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.beam_compression_range = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_beam_precompression_time => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.beam_compression_time = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_break_group => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            beam.break_group = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::beam_break_group_type => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            beam.break_group_type = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::beam_name => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            beam.name = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::beam_damp_cuttoff_hz => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.damp_cutoff_hz = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_deform_limit => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.deform_limit = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_deform_limit_expansion => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.deform_limt_expansion = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_optional => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::boolean {
                            beam.optional = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_deform_group => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            beam.deform_group = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::beam_deformation_trigger_ratio => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.deformation_trigger_ratio = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_sound_file => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            beam.sound_file = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::beam_color_factor => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.color_factor = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_attack_factor => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.attack_factor = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_volume_factor => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.volume_factor = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_decay_factor => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.decay_factor = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_pitch_factor => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.pitch_factor = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_max_stress => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.max_stress = rule.as_str().parse().unwrap();
                        }
                    }
                },
                _=> ()
            }
        }
    }

    beam

}

pub fn parse_beams(unparsed_file: String, nodes: &Vec<JNode>) -> (Vec<JBeam>, Vec<JBeam>) {
    let file = JBeamParser::parse(Rule::parts, &unparsed_file).expect("Failed to parse JBeam!").next().unwrap();

    let mut beams: Vec<JBeam> = Vec::new();

    let mut invalid_beams: Vec<JBeam> = Vec::new();

    let mut beam = JBeam::new();

    for rule in file.into_inner() {
        if rule.as_rule() == Rule::beams {


            

            for rule in rule.into_inner() {
                match rule.as_rule() {
                    Rule::beam => {


                        for rule in rule.into_inner() {
                            match rule.as_rule() {
                                Rule::string => {
                                    if beam.id1.is_empty() {
                                        beam.id1 = rule.as_str().to_string().replace(r#"""#, "");
                                    } else {
                                        beam.id2 = rule.as_str().to_string().replace(r#"""#, "");
                                    }
                                },
                                Rule::beam_modifiers => {
                                    beam = parse_beam_modifiers(rule.into_inner(), beam);
                                }
                                _=> ()
                            }
                        }

                        // check if nodes exist

                        let mut node1_exists = false;
                        let mut node2_exists = false;

                        for node in nodes {
                            if node.id == beam.id1 {
                                node1_exists = true;
                            }
                            if node.id == beam.id2 {
                                node2_exists = true;
                            }
                        }

                        if node1_exists && node2_exists {
                            beams.push(beam.clone());
                            println!("Valid beam");
                        } else {
                            invalid_beams.push(beam.clone());
                            println!("Invalid beam");
                        }



                        beam = JBeam::new();

                    },
                    Rule::beam_modifiers => {
                        beam = parse_beam_modifiers(rule.into_inner(), beam);
                    },
                    _ => ()
                }
            }
        }
    }



    (beams, invalid_beams)
}

pub fn get_distance(a: Vec3, b: Vec3) -> f32 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    let z = a.z - b.z;

    let distance = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();

    distance
}

pub fn get_closest_node_index(nodes: &Vec<JNode>, pos: Vec3) -> Option<usize> {
    let mut closest_node_index: Option<usize> = None;
    let mut closest_distance: f32 = 0.0;

    for (i, node) in nodes.iter().enumerate() {
        let distance = get_distance(pos, node.position.into());

        if closest_node_index.is_none() {
            closest_node_index = Some(i);
            closest_distance = distance;
        } else {
            if distance < closest_distance {
                closest_node_index = Some(i);
                closest_distance = distance;
            }
        }
    }

    closest_node_index
}

pub fn new_node(nodes: &Vec<JNode>, pos: (f32, f32, f32), id: String) -> Option<JNode> {
    // check if there is already a node at the given position or has the same id

    for node in nodes {
        if node.id == id {
            return None;
        }

        if node.position == pos {
            return None;
        }
    }

    let mut node = JNode::new();

    node.id = id;
    node.position = pos;

    return Some(node);

}

pub fn new_beam(nodes: &Vec<JNode>, beams: &Vec<JBeam>, id1: String, id2: String) -> Option<JBeam> {
    // check if there is already a beam going from id1 to id2

    let mut node_1_found = false;
    let mut node_2_found = false;

    for node in nodes {
        if node.id == id1 {
            node_1_found = true;
        } else if node.id == id2 {
            node_2_found = true;
        }
    }

    if !node_1_found || !node_2_found {
        println!("Invalid node ids!");
        return None;
    }

    for beam in beams {
        if beam.id1 == id1 && beam.id2 == id2 {
            println!("This beam already exists!");
            return None;
            
        }
    }

    let mut beam = JBeam::new();
    beam.id1 = id1;
    beam.id2 = id2;
    beam.imported = false;

    return Some(beam);
}






pub fn write_user_created_nodes(nodes: &Vec<JNode>) {

    let mut export_string = "".to_string();

    for node in nodes {
        if !node.imported {
            export_string = format!("{}{}\n", export_string, node.write("idk".to_string()));

        }
    }
    println!("{}", export_string);
    fs::write("created_nodes.txt", export_string).expect("Could not write to file!");
}

pub fn write_user_created_beams(beams: &Vec<JBeam>) {

    let mut export_string = "".to_string();

    for beam in beams {
        if !beam.imported {
            export_string = format!("{}{}\n", export_string, beam.write());
        }
    }

    println!("{}", export_string);
    fs::write("created_beams.txt", export_string).expect("Could not write to file!");


}

