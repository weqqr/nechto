struct VertexOutput
{
};

[shader("vertex")]
VertexOutput vs_main()
{
    VertexOutput output;

    return output;
}

struct FragmentOutput
{
    float4 color;
};

[shader("fragment")]
FragmentOutput fs_main() : SV_Target
{
    FragmentOutput output;
    output.color = float4(1.0, 1.0, 1.0, 1.0);
    return output;
}
