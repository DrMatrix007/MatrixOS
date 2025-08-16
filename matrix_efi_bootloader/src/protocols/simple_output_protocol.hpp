#if !defined(MATRIX_EFI_SIMPLE_OUTPUT_PROTOCOL_H)
#define MATRIX_EFI_SIMPLE_OUTPUT_PROTOCOL_H

#include "protocol.hpp"

namespace mst
{
    using raw_simple_output_protocol = EFI_SIMPLE_TEXT_OUT_PROTOCOL;

    class simple_output_protocol
    {
    public:
        simple_output_protocol(raw_simple_output_protocol* ptr);



        
    private:
        raw_simple_output_protocol *m_raw;
    };

}

#endif // MATRIX_EFI_SIMPLE_OUTPUT_PROTOCOL_H
